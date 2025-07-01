fn main() {
    panic!("Execute \"cargo test\" to test the contract, not \"cargo run\".");
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use casper_engine_test_support::{
        DeployItemBuilder, ExecuteRequestBuilder, LmdbWasmTestBuilder, ARG_AMOUNT,
        DEFAULT_ACCOUNT_ADDR, DEFAULT_PAYMENT, LOCAL_GENESIS_REQUEST,
    };
    use casper_execution_engine::{engine_state::Error, execution::ExecError};
    use casper_types::{runtime_args, ApiError, Key, RuntimeArgs};

    // Define `KEY` constant to match that in the contract.
    const KEY: &str = "my-key-name";
    const VALUE: &str = "hello world";
    const RUNTIME_ARG_NAME: &str = "message";
    const CONTRACT_WASM: &str = "contract.wasm";

    #[test]
    fn should_store_hello_world() {
        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();

        // The test framework checks for compiled Wasm files in '<current working dir>/wasm'.  Paths
        // relative to the current working dir (e.g. 'wasm/contract.wasm') can also be used, as can
        // absolute paths.
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = runtime_args! {
            RUNTIME_ARG_NAME => VALUE,
        };

        let deploy_item = DeployItemBuilder::new()
            .with_standard_payment(runtime_args! {
                ARG_AMOUNT => *DEFAULT_PAYMENT
            })
            .with_session_code(session_code, session_args)
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(&deploy_item).build();

        // prepare assertions.
        let result_of_query = builder.query(
            None,
            Key::Account(*DEFAULT_ACCOUNT_ADDR),
            &[KEY.to_string()],
        );
        assert!(result_of_query.is_err());

        // deploy the contract.
        builder.exec(execute_request).commit().expect_success();

        // make assertions
        let result_of_query = builder
            .query(
                None,
                Key::Account(*DEFAULT_ACCOUNT_ADDR),
                &[KEY.to_string()],
            )
            .expect("should be stored value.")
            .as_cl_value()
            .expect("should be cl value.")
            .clone()
            .into_t::<String>()
            .expect("should be string.");

        assert_eq!(result_of_query, VALUE);
    }

    #[test]
    fn should_error_on_missing_runtime_arg() {
        let session_code = PathBuf::from(CONTRACT_WASM);
        let session_args = RuntimeArgs::new();

        let deploy_item = DeployItemBuilder::new()
            .with_standard_payment(runtime_args! {ARG_AMOUNT => *DEFAULT_PAYMENT})
            .with_authorization_keys(&[*DEFAULT_ACCOUNT_ADDR])
            .with_address(*DEFAULT_ACCOUNT_ADDR)
            .with_session_code(session_code, session_args)
            .build();

        let execute_request = ExecuteRequestBuilder::from_deploy_item(&deploy_item).build();

        let mut builder = LmdbWasmTestBuilder::default();
        builder.run_genesis(LOCAL_GENESIS_REQUEST.clone()).commit();
        builder.exec(execute_request).commit().expect_failure();

        let actual_error = builder.get_error().expect("must have error");
        assert!(
            matches!(
                actual_error,
                Error::Exec(ExecError::Revert(ApiError::MissingArgument))
            ),
            "Expected {:?}, received {:?}",
            Error::Exec(ExecError::Revert(ApiError::MissingArgument)),
            actual_error
        );
    }
}
