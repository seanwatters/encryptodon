# Encryptodon 🐘🕶️

Pachyderm Goes Private

## Encryption

### Rust

```rust
let your_keys = encryptodon::generate_keys();
let their_keys = encryptodon::generate_keys();

// your end
let status = "i toot privately 🐘💨".to_string();
let encrypted_status = encryptodon::encrypt(status.clone(), their_keys.public(), your_keys.private()).unwrap();

// their end
let decrypted_status = encryptodon::decrypt(encrypted_status, your_keys.public(), their_keys.private()).unwrap();

assert_eq!(decrypted_status, status);
```

### JavaScript

```js
import init, { decrypt, encrypt, generate_keys } from "https://unpkg.com/encryptodon@0.1.6/encryptodon.js";

(async () => {
    await init();

    const your_keys = generate_keys();
    const their_keys = generate_keys();

    // your end
    const status = "i'm a sneaky elephant 🐘👀";
    const encrypted_status = encrypt(status, their_keys.public, your_keys.private);

    // their end
    const decrypted_status = decrypt(encrypted_status, your_keys.public, their_keys.private);
    console.log(decrypted_status); // -> i'm a sneaky elephant... 🐘👀
})();
```

## Bio Parsing

## Rust

```rust
let bio = "🐘🔑:0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=\n more stuff...".to_string();

let key = encryptodon::extract_key_from_bio(bio).unwrap();

assert_eq!(key, "0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=");
```

### JavaScript

```js
import init, { extract_key_from_bio } from "https://unpkg.com/encryptodon@0.1.5/encryptodon.js";

(async () => {
    await init();

    const bio = "i eat food. 🐘🔑:0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=";

    const key = extract_key_from_bio(bio);

    console.log(key); // -> 0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=
})();
```
