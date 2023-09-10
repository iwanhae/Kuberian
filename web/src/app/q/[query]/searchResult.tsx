import FunctionCard from "@/components/functionCard";
import { SERVER_URL } from "@/config";
import { type Doc, type SearchResponse } from "@/types";

async function getData(query: string): Promise<SearchResponse> {
  const sanitized = encodeURIComponent(query);
  const res = await fetch(SERVER_URL(`/q/${sanitized}`), {
    next: { revalidate: 5 },
  });

  if (!res.ok) {
    throw new Error("Failed to fetch data");
  }

  return await res.json();
}

export default async function SearchReselt({
  query,
}: {
  query: string;
}): Promise<JSX.Element> {
  const data = await getData(query);
  const docs: Record<string, Doc> = data.docs.reduce<Record<string, Doc>>(
    (prev, cur) => {
      prev[cur.id] = cur;
      return prev;
    },
    {},
  );

  return (
    <>
      {data.results
        .sort((a, b) => (a.score < b.score ? 1 : -1))
        .map((e) => (
          <FunctionCard key={e.id} doc={docs[e.id]} score={e.score} />
        ))}
    </>
  );
}
