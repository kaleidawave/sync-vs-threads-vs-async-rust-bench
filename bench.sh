# Install stuff
brew install hyperfine
brew install caddy

# Build binaries

cargo build --release
mv target/release/main sync

cargo build --release --features threads
mv target/release/main threads

cargo build --release --features async
mv target/release/main async

cargo build --release --features async,threads
mv target/release/main async_threads

# Run benchmarks

echo '# Benchmarks:' >> $GITHUB_STEP_SUMMARY

caddy start

for i in 100 1000 10000 50000 
do
    export NUM_REQUESTS=$i

    hyperfine ./sync ./threads ./async ./async_threads --export-markdown temp.md

    echo "### $i requests:" >> $GITHUB_STEP_SUMMARY
    cat temp.md >> $GITHUB_STEP_SUMMARY
done

caddy stop