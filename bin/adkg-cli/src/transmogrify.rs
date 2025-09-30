use alloy::hex;
use ark_bn254::G2Affine;
use clap::Parser;
use strum::EnumString;
use utils::serialize::point::{PointDeserializeCompressed, PointSerializeUncompressed};

#[derive(Parser, Debug)]
pub(crate) struct TransmogrifyArgs {
    #[arg(
        short = 'p',
        help = "a base64-encoded public key from the ADKG ceremony"
    )]
    public_key: String,

    #[arg(short = 'i', help = "the (expected) input format of your public key")]
    input_format: InputFormat,

    #[arg(short = 'o', help = "the desired output format")]
    output_format: OutputFormat,
}

#[derive(Copy, Clone, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum InputFormat {
    G2,
}
#[derive(Copy, Clone, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
enum OutputFormat {
    Solidity,
}

pub(crate) fn transmogrify(args: TransmogrifyArgs) -> anyhow::Result<()> {
    println!("{}", transmog_g2_solidity(&args.public_key)?);
    Ok(())
}

fn transmog_g2_solidity(public_key: &str) -> anyhow::Result<String> {
    let point = G2Affine::deser_compressed_base64(public_key)?;
    Ok(hex::encode(point.ser_uncompressed()?))
}

#[cfg(test)]
mod test {
    use crate::transmogrify::transmog_g2_solidity;

    #[test]
    fn pk_to_solidity_conformance() -> anyhow::Result<()> {
        let input = "486nH+6k3O56JiJs684/W/p8ryAipRluAplIG1ARaRkc5bjdvnnsLUni9VimypUsKm5gfW6aXSQGywYQ3Tu2zQ==";
        let expected_output = "23cea71feea4dcee7a26226cebce3f5bfa7caf2022a5196e0299481b501169191ce5b8ddbe79ec2d49e2f558a6ca952c2a6e607d6e9a5d2406cb0610dd3bb6cd2638f452d5d029d83e3cd216ffa983b38011646cbf96d2b30956b92ae5836c3913ae657a3c88cbd18ebc7d4b1e577b4faedc71f9ace79db7823b8efa20bbf3f3";
        let result = transmog_g2_solidity(input)?;

        assert_eq!(result, expected_output);

        Ok(())
    }
}
