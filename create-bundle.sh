if [ -d "release" ]; then
    rm -rf release
fi

mkdir release
cargo lambda build --release --arm64
#!/bin/bash

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

        # Change the current directory to the lambda directory
        cd "$lambda_dir"

        # Compress the lambda files to a ZIP file with the same name
        zip -rj "$OLDPWD/$release_dir/$lambda_name.zip" .

        # Change the current directory back to the previous directory
        cd "$OLDPWD"

        # Print a message with the lambda name and release path
        echo "Released lambda '$lambda_name' to '$PWD/$release_dir/$lambda_name.zip'"
    fi
done
