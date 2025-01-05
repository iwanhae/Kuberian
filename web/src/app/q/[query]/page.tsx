import SearchReselt from "./searchResult";

export const runtime = "edge";

export default async function Page({
  params,
}: {
  params: Promise<{ query: string }>;
}): Promise<JSX.Element> {
  const query = (await params).query;
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
