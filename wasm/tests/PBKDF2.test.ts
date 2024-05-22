// Copyright © 2017-2022 Trust Wallet.
//
// This file is part of Trust. The full Trust copyright notice, including
// terms governing use, modification, and redistribution, is contained in the
// file LICENSE at the root of the source code distribution tree.

import "mocha";
import { assert } from "chai";
import { Buffer } from "buffer";

describe("PBKDF2", () => {
  it("test sha256 hmac", () => {
    const { PBKDF2, HexCoding } = globalThis.core;

    const password = Buffer.from("password");
    const salt = Buffer.from("salt");
    
    const key = PBKDF2.hmacSha256(password, salt, 1, 20);
    const key2 = PBKDF2.hmacSha256(password, salt, 4096, 20);

    assert.equal(HexCoding.encode(key), "0x120fb6cffcf8b32c43e7225256c4f837a86548c9");
    assert.equal(HexCoding.encode(key2), "0xc5e478d59288c841aa530db6845c4c8d962893a0");
  });
});
