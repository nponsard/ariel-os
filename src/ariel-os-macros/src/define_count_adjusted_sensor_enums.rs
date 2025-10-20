/// Generates sensor-related enums whose number of variants needs to be adjusted based on Cargo
/// features, to accommodate the sensor driver returning the largest number of samples.
///
/// One single type must be defined so that it can be used in the Future returned by sensor
/// drivers, which must be the same for every sensor driver so it can be part of the `Sensor`
/// trait.
#[proc_macro]
pub fn define_count_adjusted_sensor_enums(_item: TokenStream) -> TokenStream {
    use quote::quote;

    #[allow(clippy::wildcard_imports)]
    use define_count_adjusted_enum::*;

    let count = get_allocation_size();

    let samples_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([Sample; #i]) }
    });
    let samples_new_funcs = (1..=count).map(|i| {
        let variant = variant_name(i);
        let func_name = new_variant_func_name(i);
        quote! {
            /// Creates a new [`Samples`] containing `#i` samples.
            pub fn #func_name(value: [Sample; #i], sensor: &'static dyn Sensor) -> Self {
                    Self {
                    samples: InnerSamples::#variant(value),
                    sensor,
                }
            }
        }
    });
    let samples_first_sample = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            InnerSamples::#variant(samples) => {
                if let Some(sample) = samples.first() {
                    *sample
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
        }
    });

    let reading_channels_from_impls = (1..=count)
        .map(|i| {
            let variant = variant_name(i);
            quote! {
                impl From<[ReadingChannel; #i]> for ReadingChannels {
                    fn from(value: [ReadingChannel; #i]) -> Self {
                        Self { channels: InnerReadingChannels::#variant(value) }
                    }
                }
            }
        });
    let reading_channels_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([ReadingChannel; #i]) }
    });

    let samples_iter = (1..=count)
        .map(|i| {
            let variant = variant_name(i);
            quote! { InnerSamples::#variant(ref samples) => samples.iter().copied() }
        });
    let reading_channels_iter = (1..=count)
        .map(|i| {
            let variant = variant_name(i);
            quote! { InnerReadingChannels::#variant(ref channels) => channels.iter().copied() }
        });

    let expanded = quote! {
        /// For driver implementors only, access to the sensor.
        pub trait SensorAccess {
            /// Get the sensor that produced these samples. For driver implementors only.
            fn sensor(&self) -> &'static dyn Sensor;
        }

        #[derive(Copy, Clone, Debug)]
        struct SizedIterator<I: Iterator> {
            iter: I,
            size: usize,
        }

        impl<I: Iterator> SizedIterator<I> {
            fn new(iter: I, size: usize) -> Self {
                Self { iter, size }
            }
        }

        impl<I: Iterator> Iterator for SizedIterator<I> {
            type Item = I::Item;

            fn next(&mut self) -> Option<Self::Item> {
                self.iter.next()
            }
            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.size, Some(self.size))
            }
        }

        impl<I: Iterator> ExactSizeIterator for SizedIterator<I> {}
        impl<I: Iterator + core::iter::FusedIterator> core::iter::FusedIterator for SizedIterator<I> {}

        /// Return all samples of a sensor, without filtering out opaque channels. For driver implementors only.
        pub trait UnfilteredSamples {
            /// Return all samples of a sensor, without filtering out opaque channels. For driver implementors only.
            fn unfiltered_samples(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator;
        }

        /// Samples returned by a sensor driver.
        ///
        /// This type implements [`Reading`] to iterate over the samples.
        ///
        /// # Note
        ///
        /// This type is automatically generated, the number of [`Sample`]s that can be stored is
        /// automatically adjusted.
        #[derive(Copy, Clone)]
        pub struct Samples {
            samples: InnerSamples,
            sensor: &'static dyn Sensor,
        }

        impl core::fmt::Debug for Samples {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("Samples")
                 .field("samples", &self.samples)
                 .finish()
            }
        }

        impl Samples {
            #(#samples_new_funcs)*
        }

        impl SensorAccess for Samples {
            fn sensor(&self) -> &'static dyn Sensor {
                self.sensor
            }
        }

        impl UnfilteredSamples for Samples {
            fn unfiltered_samples(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator {
                match self.samples {
                    #(#samples_iter),*
                }
            }
        }

        impl Reading for Samples {
            fn sample(&self) -> Sample {
                match self.samples {
                    #(#samples_first_sample),*
                }
            }

            fn samples(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator {
                let size = self.sensor().reading_channels().iter().len();
                let iter = self.unfiltered_samples()
                    .enumerate()
                    .filter(move |(i,_sample)| {
                        if let Some(channel) = self.sensor().reading_channels().iter_raw().nth(*i){
                            channel.label() != Label::Opaque
                        } else {
                            false
                        }
                    })
                    .map(|(_,sample)| sample);
                SizedIterator::new(iter, size)
            }
        }

        #[derive(Debug, Copy, Clone)]
        enum InnerSamples {
            #(#samples_variants),*
        }

        /// Metadata required to interpret samples returned by [`Sensor::wait_for_reading()`].
        ///
        /// # Note
        ///
        /// This type is automatically generated, the number of [`ReadingChannel`]s that can be
        /// stored is automatically adjusted.
        #[derive(Debug, Copy, Clone)]
        pub struct ReadingChannels {
            channels: InnerReadingChannels,
        }

        #(#reading_channels_from_impls)*

        impl ReadingChannels {
            /// Returns an iterator over the underlying [`ReadingChannel`] items.
            ///
            /// For a given sensor driver, the number and order of items match the one of
            /// [`Samples`].
            /// [`Iterator::zip()`] can be useful to zip the returned iterator with the one
            /// obtained with [`Reading::samples()`].
            pub fn iter(&self) -> impl ExactSizeIterator<Item = ReadingChannel> + core::iter::FusedIterator + '_ {
                let iter = self.iter_raw()
                    .filter(|channel| channel.label() != Label::Opaque);

                let cloned = self.iter_raw()
                    .filter(|channel| channel.label() != Label::Opaque);

                let size = cloned.count();

                SizedIterator::new(iter, size)
            }

            pub(crate) fn iter_raw(&self) -> impl ExactSizeIterator<Item = ReadingChannel> + core::iter::FusedIterator + '_ {
                match self.channels {
                    #(#reading_channels_iter),*
                }
            }

            /// Returns the first [`ReadingChannel`].
            pub fn first(&self) -> ReadingChannel {
                if let Some(sample) = self.iter().next() {
                    sample
                } else {
                    // NOTE(no-panic): there is always at least one sample.
                    unreachable!();
                }
            }
        }

        #[derive(Debug, Copy, Clone)]
        enum InnerReadingChannels {
            #(#reading_channels_variants),*
        }
    };

    TokenStream::from(expanded)
}

mod define_count_adjusted_enum {
    pub fn variant_name(index: usize) -> syn::Ident {
        quote::format_ident!("V{index}")
    }

    pub fn new_variant_func_name(index: usize) -> syn::Ident {
        quote::format_ident!("new_{index}")
    }

    pub fn get_allocation_size() -> usize {
        // The order of these feature-gated statements is important as these features are not meant to
        // be mutually exclusive.
        #[allow(unused_variables, reason = "overridden by feature selection")]
        let count = 1;
        #[cfg(feature = "max-sample-min-count-2")]
        let count = 2;
        #[cfg(feature = "max-sample-min-count-3")]
        let count = 3;
        #[cfg(feature = "max-sample-min-count-4")]
        let count = 4;
        #[cfg(feature = "max-sample-min-count-5")]
        let count = 5;
        #[cfg(feature = "max-sample-min-count-6")]
        let count = 6;
        #[cfg(feature = "max-sample-min-count-7")]
        let count = 7;
        #[cfg(feature = "max-sample-min-count-8")]
        let count = 8;
        #[cfg(feature = "max-sample-min-count-9")]
        let count = 9;
        #[cfg(feature = "max-sample-min-count-12")]
        let count = 12;

        count
    }
}
