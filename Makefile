integration_tests:
	cdk build -p sql-sink
	RUST_LOG=warn,integration_tests=info cargo run -p integration-tests

build:
	cdk build -p sql-sink

test:
	cdk test  --release release  -p sql-sink --config mqtt-sql.yaml --secrets .env

test_local:
	cdk test  --release release  -p sql-sink --config mqtt-duck.yaml