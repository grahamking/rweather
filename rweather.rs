
extern mod std;

use std::{net_tcp,net_ip};
use std::uv;

fn fetch(code: ~str) -> Result<~[~str], net_tcp::TcpConnectErrData> {

    let ipaddr = net_ip::v4::parse_addr("205.156.51.232");
    let iotask = uv::global_loop::get();
    let connect_result = net_tcp::connect(ipaddr, 80, &iotask);
    let sock;

    let data_get = fmt!(
        "GET /pub/data/observations/metar/decoded/%s.TXT\n HTTP/1.0",
        code.to_ascii().to_upper().to_str_ascii());
    let data_headers = "Host: weather.noaa.gov\n\n";

    match connect_result {
        Ok(socket) => { sock = net_tcp::socket_buf(socket); }
        Err(e) => { println(fmt!("%?", e)); return Err(e); }
    }

    sock.write(data_get.to_bytes());
    sock.write(data_headers.to_bytes());

    let output = sock.read_lines();

    return Ok(output);
}

fn display(data: ~[~str]) {

    for data.each |&line| {
        println(line);
    }
}

fn main() {

    let help = "Usage: rweather <code>.\n The code is your ICAO airport code - see http://en.wikipedia.org/wiki/List_of_airports_by_ICAO_code.\n In the US it's the normal airport code prefixed with a K, e.g. KLAX for Los Angeles, KSFO for San Francisco. In Canada it's the airport code prefixed with a C - CYVR for Vancouver.";

    let args = os::args();

    if args.len() != 2 {
        println(help);
        return;
    }

    let code = copy args[1];

    let result = fetch(code);
    match result {
        Ok(data) => { display(data); }
        Err(e) => { println(fmt!("%?", e)); }
    }

}
