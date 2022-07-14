export function displayBanner(message: string, exception?: Error) {
  const kind = exception ? "Error" : "Info";
  if (exception) console.log(exception);
  alert(`[${kind}]: ${message} \n${exception ? exception : ""}`);
}
