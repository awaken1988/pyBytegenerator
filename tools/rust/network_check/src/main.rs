use std::{net::{IpAddr, Ipv4Addr, Ipv6Addr}, time::Duration, thread, time::SystemTime, iter, fs, fs::File, io::Write};
use dns_lookup::{lookup_host, lookup_addr};
use ping::ping;

struct CheckerState<'a>
{
    idx:  usize,
    addresses: &'a Vec<IpAddr>,

}

impl<'a> CheckerState<'a> {
    fn new(addresses: &'a Vec<IpAddr>) -> CheckerState {
        return CheckerState {
            idx: addresses.len(),
            addresses: addresses,
        };
    }

    fn next_id(&mut self) -> usize {
        self.idx+=1;
        if self.idx >= self.addresses.len() {
            self.idx=0;
        }
            
        return self.idx
    } 

    fn check(&mut self)-> Option<IpAddr> {
        for i_round in 0..self.addresses.len() {
            let ip_addr = self.addresses[self.next_id()];

            let ping_result = ping(
                ip_addr,
                Some(Duration::from_secs(2)),
                Option::None,
                Option::None,
                Option::None,
                Option::None);

           

            if let Ok(()) = ping_result {
                return Some(ip_addr);
            }
        }

        return None;
    }
}

fn main() {

    let hostnames = vec![
        "www.google.de".to_string(),
        "www.heise.de".to_string(),
        "www.chip.de".to_string(),
        "8.8.8.8".to_string(),
        "8.8.4.4".to_string(),
        "9.9.9.9".to_string(),
        "1.1.1.1".to_string(),
        "2001:4860:4860::8888".to_string(),
        "2001:4860:4860::8844".to_string(),
    ];

    let addresses = hostnames.iter()
        .map(|x|{ lookup_host(x) })
        .map_while(|x| if let Ok(y) = x {Some(y)} else { Option::None }  )
        .flatten()
        .collect::<Vec<IpAddr>>();

    println!("Test Ips:");
    for address in addresses.iter() {
        println!("    {}", address.to_string()); 
    }


    let mut checker = CheckerState::new(&addresses);

    let mut outfile = File::create("out.txt").expect("cannot open logfile");

    loop {
        let sys_time           = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).expect("cannot get unixtime");
        let is_connected = checker.check();

        let mut output = format!("{}", sys_time.as_secs());

        if let Some(ip_addr) = checker.check() {
            output = format!("{} true {}", output, ip_addr.to_string());
        }
        else {
            output = format!("{} false", output);
        }

        println!("{}", output);
        outfile.write(format!("{}\n",output).as_bytes());

        thread::sleep(Duration::from_secs(4));
    }



    // for i in addresses {
    //        match i {
    //         IpAddr::V4(x) => println!("v4 {}", i),
    //         IpAddr::V6(x) => println!("v6 {}", i),
    //        }
    // }


    // let x = ping(
    //     IpAddr::from([127,0,0,1,]),
    //     Some(Duration::from_secs(3)),
    //     Option::None,
    //     Option::None,
    //     Option::None,
    //     Option::None).unwrap();
    // let result=lookup_host("8.8.8.8").unwrap();
    // for i in result.iter() {
    //     match i {
    //         IpAddr::V4(x) => println!("v4 {}", i),
    //         IpAddr::V6(x) => println!("v6 {}", i),
    //     }
    // }


    println!("Hello, world!");
}
