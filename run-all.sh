cargo build --release && \
for bin in $(cat Cargo.toml | grep -oP '(?<=name = ")(.*)(?=")' | grep -vE 'aoclib|name'); do \
    if [ -x "target/release/$bin" ]; then \
        echo $'\n'
        echo "Solving $bin..."
        ./target/release/$bin
    fi
done