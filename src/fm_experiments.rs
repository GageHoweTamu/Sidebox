// FM (TODO)
                    /*
                    // make vectors to hold the samples and their locations
                    let mut samples_buffer = Buffer::new();
                    let mut locations_left_vec = Vec::new(); // locations of samples in the samples vector (float)
                    let mut tmp_buffer = Buffer::new(); // where we hold the interpolated samples
                    let mut phase_offset = 0.0;
                
                    // for each sample in the channel and sidechain, push the sample and the phase offset into the vectors
                    // the location is the phase offset plus the index of the sample
                    let mut index = 0.0;
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {

                        phase_offset_right = *sidechain_sample * sidechain_input_gain; // set phase offset to the value of the sidechain sample

                        samples_vec.push(*sample); // push the sample
                        locations_vec.push(phase_offset+xindex); // push the location of the sample; the location is the phase offset plus the index of the sample

                        index += 1.0;
                    }

                    // interpolate the samples linearly
                    // for each element in samples, interpolate the value at the location in the output buffer.
                    // for example, if the location is 1.5, the value at the location in the output buffer is 0.5*sample[1] + 0.5*sample[2]
                    // however, we can't directly access the index of sample and sidechain_sample

                    let mut output_buffer = vec![0.0; channel_samples.len()];

                    for j in 0..samples.len() {
                        let sample = samples[j];
                        let location = locations[j];
                        let index = location as usize;
                        let frac = location - index as f32;
                        if index < output_buffer.len() - 1 {
                            output_buffer[index] += (1.0 - frac) * sample;
                            output_buffer[index + 1] += frac * sample;
                        }
                    }
                        // populate the channel
                    for (channel_sample, output_sample) in channel_samples.iter_mut().zip(output_buffer.iter()) {
                        *channel_sample = *output_sample;
                    }
                }
                2 => { // weird fm thing
                    // declare a variable to hold the phase of the sidechain signal
                    let mut phase = 0.0;
                    // declare a buffer to hold the samples
                    let mut buffer = Vec::new();
                
                    for (sample, sidechain_sample) in channel_samples.iter_mut().zip(sidechain_samples.iter_mut()) {
                        // increment the phase by the sidechain sample multiplied by the sidechain input gain
                        phase += *sidechain_sample * sidechain_input_gain;
                
                        // push the sample into the buffer
                        buffer.push(*sample * input_gain);
                
                        // calculate the index of the sample to play back
                        let index = (buffer.len() as f32 * phase) as usize;
                
                        // make sure the index is within the bounds of the buffer
                        if index < buffer.len() {
                            // replace the sample with the one from the buffer
                            *sample = buffer[index];
                        }
                    }
                }*/