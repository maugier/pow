use {
    crypto::{
        digest::Digest,
        md5::Md5,
        sha1::Sha1,
        sha2::{
            Sha224,
            Sha256,
            Sha384,
            Sha512,
        },
    },
    rand::{
        distributions::Alphanumeric,
        Rng,
        thread_rng,
    },
    std::env,
};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        panic!("Usage: ./pow <algo> <len> <target>");
    }

    let len: usize = args[2].parse().expect("length must be a number");
    let target = hex::decode(args[3].as_bytes()).expect("Cannot understand target");

    println!("Attacking {:?} {}", target, len);

    let answer: String = match args[1].as_ref() {
        "md5" => brute(Md5::new(), len, &target),
        "sha1" => brute(Sha1::new(), len, &target),
        "sha224" => brute(Sha224::new(), len, &target),
        "sha256" => brute(Sha256::new(), len, &target),
        "sha384" => brute(Sha384::new(), len, &target),
        "sha512" => brute(Sha512::new(), len, &target),
        _ => panic!("Unknown hash algo"),
    };

    println!("{}", answer);

}


fn brute<H: Digest>(mut hash: H, len: usize, target: &[u8]) -> String {
    let mut rng = thread_rng();
    let outlen = hash.output_bytes();
    let tlen = target.len();

    let mut out = vec![0; outlen];

    loop {
        let s: String = (0..len)
            .map(|_| rng.sample(Alphanumeric))
            .collect();

        hash.input(s.as_bytes());
        hash.result(&mut out);

        if &out[(outlen - tlen)..] == target {
            return s
        }

        hash.reset();
    }
}
