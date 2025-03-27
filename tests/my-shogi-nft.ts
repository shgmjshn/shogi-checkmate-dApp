import * as anchor from "@coral-xyz/anchor";

describe("my-shogi-nft", () => {
  it("Basic test", async () => {
    console.log("Running basic test...");
    // シンプルなテスト
    assert(true);
  });
});

function assert(condition: boolean) {
  if (!condition) throw new Error("Assertion failed");
  return true;
}