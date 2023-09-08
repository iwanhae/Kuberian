import SearchReselt from "./searchResult";

export const runtime = "edge";

export default async function Page({
  params: { query },
}: {
  params: { query: string };
}): Promise<JSX.Element> {
  return (
    <>
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 flex flex-col gap-5">
          <SearchReselt query={query} />
        </div>
      </main>
    </>
  );
}
