Displays the local weather information. Mostly I'm playing with Rust.

## Build

You need to have [Rust](http://www.rust-lang.org/) (incoming branch), then just `rustc rweather.rs` to build it.

## Usage

From the command line: `rweather <code>`

The code is any code that weather.noaa.gov will return a value for. Try your local [ICAO airport code](http://en.wikipedia.org/wiki/List_of_airports_by_ICAO_code). In the US it's your normal airport code prefixed with a K, e.g. _KLAX_ for Los Angeles, _KSFO_ for San Francisco, etc. In Canada it's the airport code prefixed with a C - _CYVR_ for Vancouver.

