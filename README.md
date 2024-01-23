# Encryptodon

Pachyderm Goes Private 🐘🕶️

## Encryption

```javascript
// JavaScript (web)

import init, { decrypt, encrypt, generate_keys } from "https://unpkg.com/encryptodon@0.1.7/encryptodon.js";

(async () => {
    await init();

    const your_keys = generate_keys();
    const their_keys = generate_keys();

    // your end
    const status = "i'm a sneaky elephant 🐘👀";
    const encrypted_status = encrypt(status, their_keys.public, your_keys.private);

    // their end
    const decrypted_status = decrypt(encrypted_status, your_keys.public, their_keys.private);
    console.log(decrypted_status); // -> i'm a sneaky elephant 🐘👀
})();
```

```rust
// Rust (embedded)

let your_keys = encryptodon::generate_keys();
let their_keys = encryptodon::generate_keys();

// your end
let status = "i toot privately 🐘💨".to_string();
let encrypted_status = encryptodon::encrypt(status.clone(), their_keys.public(), your_keys.private()).unwrap();

// their end
let decrypted_status = encryptodon::decrypt(encrypted_status, your_keys.public(), their_keys.private()).unwrap();

assert_eq!(decrypted_status, status);
```

## Bio Parsing

```javascript
// JavaScript (web)

import init, { extract_key_from_bio } from "https://unpkg.com/encryptodon@0.1.7/encryptodon.js";

(async () => {
    await init();

    const bio = "i eat food. 🐘🔑:0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=";

    const key = extract_key_from_bio(bio);

    console.log(key); // -> 0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=
})();
```

```rust
// Rust (embedded)

let bio = "🐘🔑:0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=\nmore stuff...".to_string();

let key = encryptodon::extract_key_from_bio(bio).unwrap();

assert_eq!(key, "0bmKKWS04lZzoPC/KlS1kJgWN+XnvBw0yn4PPnot73E=");
```
