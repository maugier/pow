use {
    digest::{Digest},
    md5::Md5,
    sha1::Sha1,
    sha2::{Sha224,Sha256,Sha384,Sha512},

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

    let answer: String = match args[1].as_ref() {
        "md5"    => brute::<Md5>(len, &target),
        "sha1"   => brute::<Sha1>(len, &target),
        "sha224" => brute::<Sha224>(len, &target),
        "sha256" => brute::<Sha256>(len, &target),
        "sha384" => brute::<Sha384>(len, &target),
        "sha512" => brute::<Sha512>(len, &target),
        _ => panic!("Unknown hash algo"),
    };

    println!("{}", answer);

}


fn brute<H: Digest>(len: usize, target: &[u8]) -> String {
    let mut rng = thread_rng();
    let outlen = <H as Digest>::output_size();
    let tlen = target.len();

    loop {
        let s: String = (0..len)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();

        let out = H::digest(&s);

        if &out[(outlen - tlen)..] == target {
            return s
        }
    }
}

#[test]
fn try_short_target() {
    let len = 10;
    let target = [0x1a, 0xb2];
    let solution = brute(Sha256::new(), 10, &target);

    assert_eq!(solution.len(), len);

    let mut out = [0; 32];
    let mut hash = Sha256::new();
    hash.input(solution.as_bytes());
    hash.result(&mut out);

    assert_eq!(&out[30..32], &target);

}
