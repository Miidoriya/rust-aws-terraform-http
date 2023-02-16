if [ -d "release" ]; then
    rm -rf release
fi

mkdir release
cd target/lambda/basic-lambda
zip index.zip bootstrap
mv index.zip ../../../release