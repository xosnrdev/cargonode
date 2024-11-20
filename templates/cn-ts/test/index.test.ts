import { expect, it } from "vitest";

import { add } from "../src/main";

it("should add two numbers", () => {
    expect(add(1, 2)).toBe(3);
    expect(add(2, 3)).toBe(5);
});
