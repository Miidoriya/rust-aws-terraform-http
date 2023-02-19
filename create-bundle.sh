if [ -d "release" ]; then
    rm -rf release
fi

mkdir release
cargo lambda build --release --arm64
cd target/lambda/http-lambda
zip index.zip bootstrap
mv index.zip ../../../release