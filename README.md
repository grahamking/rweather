Displays the local weather information. Mostly I'm playing with Rust.

## Build

You need to have [Rust](http://www.rust-lang.org/) (incoming branch), then just `rustc rweather.rs` to build it.

## Usage

From the command line: `rweather <code>`

The code is any code that [weather.noaa.gov](http://weather.noaa.gov/pub/data/observations/metar/decoded/) will return a value for. Try your local [ICAO airport code](http://en.wikipedia.org/wiki/List_of_airports_by_ICAO_code). In the US it's your normal airport code prefixed with a K, e.g. _KLAX_ for Los Angeles, _KSFO_ for San Francisco, etc. In Canada it's the airport code prefixed with a C - _CYVR_ for Vancouver.

Example:

You type: `rweather CYVR`

Output:

    Vancouver International Air-Port, B. C., Canada (CYVR) 49-11N 123-10W 2M
    Apr 25, 2013 - 01:00 AM EDT / 2013.04.25 0500 UTC
    Wind: Calm:0
    Visibility: 30 mile(s):0
    Sky conditions: mostly cloudy
    Temperature: 55 F (13 C)
    Dew Point: 41 F (5 C)
    Relative Humidity: 58%
    Pressure (altimeter): 30.09 in. Hg (1018 hPa)
    ob: CYVR 250500Z 00000KT 30SM BKN240 13/05 A3009 RMK CI2 SLP188
    cycle: 5

-------

If you actually want something that does this for long term usage, I'd recommend [weather-util](http://fungi.yuggoth.org/weather/), which is in Ubuntu repos.
