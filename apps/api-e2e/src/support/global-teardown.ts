module.exports = async function () {
  // The API server is a continuous Nx task (`api:serve`); Nx stops it once
  // the e2e target finishes, so no manual process cleanup is needed here.
  console.log(globalThis.__TEARDOWN_MESSAGE__);
};
