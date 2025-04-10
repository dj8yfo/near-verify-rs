use near_verify_rs::types::{
    contract_source_metadata::ContractSourceMetadata, whitelist::Whitelist,
};
mod checkout;

struct TestCase {
    input: &'static str,
    expected_output: &'static str,
}
fn common_verify_test_routine_opts(
    test_case: TestCase,
    whitelist: Option<Whitelist>,
) -> eyre::Result<()> {
    let contract_source_metadata: ContractSourceMetadata = serde_json::from_str(test_case.input)?;

    assert!(contract_source_metadata.build_info.is_some());
    let source_id = near_verify_rs::types::source_id::SourceId::from_url(
        &contract_source_metadata
            .build_info
            .as_ref()
            .unwrap()
            .source_code_snapshot,
    )?;

    let (_tempdir, target_dir) = checkout::checkout(source_id)?;

    let target_dir = camino::Utf8PathBuf::from_path_buf(target_dir)
        .map_err(|err| eyre::eyre!("convert path buf {:?}", err))?;

    contract_source_metadata.validate(whitelist)?;
    let docker_build_out_wasm =
        near_verify_rs::logic::nep330_build::run(contract_source_metadata, target_dir, vec![])?;

    let result = near_verify_rs::logic::compute_hash(docker_build_out_wasm)?;

    assert_eq!(
        result.to_base58_string(),
        test_case.expected_output,
        "Artifact hash-sum mismatch"
    );

    Ok(())
}
fn common_verify_test_routine(test_case: TestCase) -> eyre::Result<()> {
    common_verify_test_routine_opts(test_case, None)
}

/// https://testnet.nearblocks.io/address/simple-package-verify-rs-ci.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-package-v1.0.0
const SIMPLE_PACKAGE_VANILLA: TestCase = TestCase {
    input: r#"
{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=e3303f0cf8761b99f84f93c3a2d7046be6f4edb5"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/e3303f0cf8761b99f84f93c3a2d7046be6f4edb5",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
    expected_output: "5KaX9FM9NtjpfahksL8TMWQk3LF7k8Sv88Qem4tGrVDW",
};

#[test]
fn test_simple_package_vanilla() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_PACKAGE_VANILLA)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/simple-package-with-features-verify-rs-ci.testnet
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-package-with-features-v1.0.0
const SIMPLE_PACKAGE_WITH_FEATURES: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked",
      "--no-default-features",
      "--features",
      "near-sdk/legacy"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=6fc35ed210d3578b301e25b3b8c11fb53767d032"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/6fc35ed210d3578b301e25b3b8c11fb53767d032",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
    expected_output: "D5YfnZPCyzdqcdjroW7TGG3GQezdQSrcRWG4mRxdHx5d",
};
#[test]
fn test_simple_package_with_features() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_PACKAGE_WITH_FEATURES)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/simple-package-with-paseed-env-verify-rs-ci.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-package-with-passed-env-v1.0.0
const SIMPLE_PACKAGE_WITH_PASSED_ENV: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked",
      "--env",
      "KEY=VALUE",
      "--env",
      "GOOGLE_QUERY=https://www.google.com/search?q=google+translate&sca_esv=3c150c50f502bc5d"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=4f593556476fb0c5d71a73e615a391a972aa586a"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/4f593556476fb0c5d71a73e615a391a972aa586a",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
    expected_output: "3fdG1ETP7SfArvdfeM9asqNfBj3HKvBK4ZV3uz3eTdzm",
};

#[test]
fn test_simple_package_with_passed_env() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_PACKAGE_WITH_PASSED_ENV)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/simple-factory-verify-rs-cia.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-factory-v1.0.0%2Bsimple-factory-product-v1.1.0
const SIMPLE_FACTORY_VANILLA: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "workspace_root_folder/factory",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=dffdd3a2a33ee3aebb0a72cdccd902f5ab69989c"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/dffdd3a2a33ee3aebb0a72cdccd902f5ab69989c",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
    expected_output: "7qhDddxfr4p39CeBvpTXWQmzzDA4HTbrWceZtaDAExjW",
};

#[test]
fn test_simple_factory_vanilla() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_FACTORY_VANILLA)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/product.simple-factory-verify-rs-cia.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-factory-v1.0.0%2Bsimple-factory-product-v1.1.0
const SIMPLE_FACTORY_VANILLA_PRODUCT: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "workspace_root_folder/product-donation",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=dffdd3a2a33ee3aebb0a72cdccd902f5ab69989c"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/dffdd3a2a33ee3aebb0a72cdccd902f5ab69989c",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.1.0"
}"#,
    expected_output: "FLXsv6msJ6dD9A6DpJX96d3q8UiDjUtyBsdnEYVnML2U",
};

#[test]
fn test_simple_factory_product_vanilla() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_FACTORY_VANILLA_PRODUCT)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/simple-factory-with-features-verify-rs-ci-a.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-factory-with-features-v1.0.0%2Bsimple-factory-product-with-features-v1.1.0
const SIMPLE_FACTORY_WITH_FEATURES: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked",
      "--no-default-features",
      "--features",
      "near-sdk/legacy"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "workspace_root_folder/factory",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=0db6242138876e591900d3c0fdac95cc74ac6e89"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/0db6242138876e591900d3c0fdac95cc74ac6e89",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
    expected_output: "6Nmb4WML7VpKmv8KCJzxMD6SQ1jjhwiVRbKYkx2Jqts1",
};

#[test]
fn test_simple_factory_with_features() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_FACTORY_WITH_FEATURES)?;
    Ok(())
}

/// https://testnet.nearblocks.io/address/product.simple-factory-with-features-verify-rs-ci-a.testnet?tab=contract
/// https://github.com/dj8yfo/verify_contracts_collection/releases/tag/simple-factory-with-features-v1.0.0%2Bsimple-factory-product-with-features-v1.1.0
const SIMPLE_FACTORY_WITH_FEATURES_PRODUCT: TestCase = TestCase {
    input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked",
      "--features",
      "near-sdk/legacy",
      "--no-default-features"
    ],
    "build_environment": "sourcescan/cargo-near:0.13.4-rust-1.85.0@sha256:a9d8bee7b134856cc8baa142494a177f2ba9ecfededfcdd38f634e14cca8aae2",
    "contract_path": "workspace_root_folder/product-donation",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=0db6242138876e591900d3c0fdac95cc74ac6e89"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/0db6242138876e591900d3c0fdac95cc74ac6e89",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.1.0"
}"#,
    expected_output: "2onZk3T9QqqNTEMwHf6EGBtLUEa4WyebtxDfYzhq5mLW",
};

#[test]
fn test_simple_factory_product_with_features() -> eyre::Result<()> {
    common_verify_test_routine(SIMPLE_FACTORY_WITH_FEATURES_PRODUCT)?;
    Ok(())
}

mod whitelist {

    use near_verify_rs::types::whitelist::Whitelist;

    use crate::{common_verify_test_routine_opts, TestCase};

    /// https://testnet.nearblocks.io/address/donation-product.repro-fct-80.testnet?tab=contract
    const CONTRACT_WITH_NONSTANDARD_IMAGE: TestCase = TestCase {
        input: r#"{
  "build_info": {
    "build_command": [
      "cargo",
      "near",
      "build",
      "non-reproducible-wasm",
      "--locked"
    ],
    "build_environment": "dj8yfo/sourcescan:0.x.x-dev-pr-262@sha256:a231d4bf975d561a06dd5357f2ac03c883e8b3b510994f3b40c9b975dcdb02ce",
    "contract_path": "",
    "source_code_snapshot": "git+https://github.com/dj8yfo/verify_contracts_collection?rev=cb100096d0eb67654857949e1ff49fff2f385012"
  },
  "link": "https://github.com/dj8yfo/verify_contracts_collection/tree/cb100096d0eb67654857949e1ff49fff2f385012",
  "standards": [
    {
      "standard": "nep330",
      "version": "1.2.0"
    }
  ],
  "version": "1.0.0"
}"#,
        expected_output: "Fa1VfSH4SYUXymJbjG4Rz3zyLpdFciKvomtgbfa9uacd",
    };

    #[test]
    fn test_simple_package_with_nonstandard_image() -> eyre::Result<()> {
        let whitelist: Whitelist = {
            let file = std::fs::read("tests/resources/whitelist_ok_nonstandard_image.json")
                .expect("no std:fs::read error");
            serde_json::from_slice(&file).expect("no serde_json::from_slice error")
        };
        common_verify_test_routine_opts(CONTRACT_WITH_NONSTANDARD_IMAGE, Some(whitelist))?;
        Ok(())
    }

    mod decline {
        use near_verify_rs::types::whitelist::Whitelist;

        use crate::{common_verify_test_routine_opts, whitelist::CONTRACT_WITH_NONSTANDARD_IMAGE};

        #[test]
        fn test_decline_simple_package_with_unexpected_image() -> eyre::Result<()> {
            let whitelist: Whitelist = {
                let file = std::fs::read("tests/resources/whitelist_err_image.json")
                    .expect("no std:fs::read error");
                serde_json::from_slice(&file).expect("no serde_json::from_slice error")
            };
            let Err(err) =
                common_verify_test_routine_opts(CONTRACT_WITH_NONSTANDARD_IMAGE, Some(whitelist))
            else {
                panic!("Expecting an error returned from `common_verify_test_routine_opts`");
            };
            println!("{:#?}", err);

            assert!(format!("{:?}", err).contains("no matching entry found for"));
            Ok(())
        }

        #[test]
        fn test_decline_simple_package_with_unexpected_command() -> eyre::Result<()> {
            let whitelist: Whitelist = {
                let file = std::fs::read("tests/resources/whitelist_err_command.json")
                    .expect("no std:fs::read error");
                serde_json::from_slice(&file).expect("no serde_json::from_slice error")
            };
            let Err(err) =
                common_verify_test_routine_opts(CONTRACT_WITH_NONSTANDARD_IMAGE, Some(whitelist))
            else {
                panic!("Expecting an error returned from `common_verify_test_routine_opts`");
            };
            println!("{:#?}", err);

            assert!(
                format!("{:?}", err).contains("must start with expected whitelist command prefix")
            );
            Ok(())
        }
    }
}
