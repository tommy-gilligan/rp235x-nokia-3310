use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SizedSample,
    FromSample, Sample
};

fn main() {
    let stream = stream_setup_for();
    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(16000));
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub current_sample_index: f32,
    pub frequency_hz: f32,
}

impl Oscillator {
    fn set_frequency(&mut self, frequency: u32) {
        self.frequency_hz = frequency as f32;
    }

    fn tick(&mut self) -> f32 {
        self.current_sample_index = (self.current_sample_index + 1.0) % self.sample_rate;
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * self.frequency_hz * two_pi / self.sample_rate).sin()
    }
}

pub fn stream_setup_for() -> cpal::Stream {
    let (_host, device, config) = host_device_setup();

    match config.sample_format() {
        cpal::SampleFormat::I8 => make_stream::<i8>(&device, &config.into()),
        cpal::SampleFormat::I16 => make_stream::<i16>(&device, &config.into()),
        cpal::SampleFormat::I32 => make_stream::<i32>(&device, &config.into()),
        cpal::SampleFormat::I64 => make_stream::<i64>(&device, &config.into()),
        cpal::SampleFormat::U8 => make_stream::<u8>(&device, &config.into()),
        cpal::SampleFormat::U16 => make_stream::<u16>(&device, &config.into()),
        cpal::SampleFormat::U32 => make_stream::<u32>(&device, &config.into()),
        cpal::SampleFormat::U64 => make_stream::<u64>(&device, &config.into()),
        cpal::SampleFormat::F32 => make_stream::<f32>(&device, &config.into()),
        cpal::SampleFormat::F64 => make_stream::<f64>(&device, &config.into()),
        sample_format => panic!("Unsupported sample format '{sample_format}'"),
    }
}

pub fn host_device_setup() -> (cpal::Host, cpal::Device, cpal::SupportedStreamConfig) {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| panic!("Default output device is not available")).unwrap();
    let config = device.default_output_config().unwrap();
    (host, device, config)
}

const SONG_TEXT: &'static str = "Wannabe:d=4, o=5, b=125:16g, 16g, 16g, 16g, 8g, 8a, 8g, 8e, 8p, 16c, 16d, 16c, 8d, 8d, 8c, e, p, 8g, 8g, 8g, 8a, 8g, 8e, 8p, c6, 8c6, 8b, 8g, 8a, 16b, 16a, g";

pub fn make_stream<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
) -> cpal::Stream where T: SizedSample + FromSample<f32> {
    let num_channels = config.channels as usize;

    let mut oscillator = Oscillator {
        sample_rate: config.sample_rate.0 as f32,
        current_sample_index: 0.0,
        frequency_hz: 440.0,
    };
    let err_fn = |err| panic!("Error building output sound stream: {}", err);

    let time_at_start = std::time::Instant::now();
    println!("Time at start: {:?}", time_at_start);

    let mut song = rtttl::Song::new(SONG_TEXT);

    let stream = device.build_output_stream(config, move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
        let time_since_start = std::time::Instant::now().duration_since(time_at_start).as_micros() as u32;
        oscillator.set_frequency(song.note_at(time_since_start).unwrap().frequency());
        process_frame(output, &mut oscillator, num_channels)
    }, err_fn, None).unwrap();

    stream
}

fn process_frame<SampleType>(
    output: &mut [SampleType],
    oscillator: &mut Oscillator,
    num_channels: usize,
) where SampleType: Sample + FromSample<f32> {
    for frame in output.chunks_mut(num_channels) {
        let value: SampleType = SampleType::from_sample(oscillator.tick());

        // copy the same value to all channels
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
