export const SERVER_URL = (path?: string): string => {
  const apiUrl =
    process.env.KUBERIAN_API ?? "https://kuberian-thoe7ww5ya-an.a.run.app";
  return `${apiUrl}${path ?? "/"}`;
};
