const { bindings } = require("@wasmer/sgp4");
const {
  Elements,
  Constants,
  ErrorOutOfRangePerturbedEccentricity,
  ErrorNegativeSemiLatusRectum,
} = require("@wasmer/sgp4/src/bindings/sgp4/sgp4.js");

const { test, expect } = require("@jest/globals");

const TEST_CASE_DATA = require("./tests/test-cases.json").list;

const resolveResult = ({ tag, val }) => {
  if (tag === "err") {
    throw val;
  }
  return val;
};

const POSITION_PRECISION = Math.pow(10, -6);
const VELOCITY_PRECISION = Math.pow(10, -9);

test("SGP4 List-State Test", async () => {
  const wasm = await bindings.sgp4();
  for (let testCase of TEST_CASE_DATA) {
    try {
      let elements = resolveResult(
        Elements.fromTle(wasm, null, testCase.line1, testCase.line2)
      );

      let constants = resolveResult(
        Constants.fromElementsAfspcCompatibilityMode(wasm, elements)
      );

      for (let state of testCase.states) {
        if ("error" in state) {
          // throw state.error;
          const { error, time } = state;
          const predictionResult =
            constants.propagateAfspcCompatibilityMode(time);

          if (predictionResult.tag === "err") {
            let { tag: errorTag, val: errorVal } = predictionResult.val;
            const { t } = errorVal;
            switch (errorTag) {
              case "out-of-range-perturbed-eccentricity":
                expect(t).toEqual(time);
                break;
              case "negative-semi-latus-rectum":
                expect(t).toEqual(time);
                break;
              default:
                throw "Unknown error";
                break;
            }
          }
          continue;
        }
        let { time, position, velocity } = state;

        let prediction = resolveResult(
          constants.propagateAfspcCompatibilityMode(time)
        );
        for (let i = 0; i < 3; i++) {
          expect(Math.abs(position[i] - prediction.position[i])).toBeLessThan(
            POSITION_PRECISION
          );
          expect(Math.abs(velocity[i] - prediction.velocity[i])).toBeLessThan(
            VELOCITY_PRECISION
          );
        }
      }
    } catch (e) {
      console.log(e);
    }
  }
});
