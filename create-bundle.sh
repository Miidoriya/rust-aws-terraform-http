# clear release if exists
if [ -d "release" ]; then
    rm -rf release
fi

# create the release directory
mkdir release

# build the various lambdas
cargo lambda build --release --arm64

# Set the target directory name pattern for lambdas
lambda_dir_pattern="target/lambda/*"

# Set the release directory path relative to the root directory
release_dir="release"

# Create the release directory if it doesn't exist
mkdir -p "$release_dir"

# Iterate over the lambda directories
for lambda_dir in $lambda_dir_pattern; do
    if [[ -d "$lambda_dir" ]]; then
        # Extract the lambda name from the directory path
        lambda_name=$(basename "$lambda_dir")

        # Compress the lambda directory to a ZIP file with the same name
        zip -r "$release_dir/$lambda_name.zip" "$lambda_dir"

        # Print a message with the lambda name and release path
        echo "Released lambda '$lambda_name' to '$PWD/$release_dir/$lambda_name.zip'"
    fi
done
