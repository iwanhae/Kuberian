import { type Doc } from "@/types";
import Card from "./card";
import Link from "next/link";
import { LinkExternalIcon } from "@primer/octicons-react";

export default function FunctionCard({
  doc,
  score,
}: {
  doc: Doc;
  score?: number;
}): JSX.Element {
  let color = "gray";
  if (score !== undefined) {
    if (score > 0.8) color = "sky";
    else if (score > 0.6) color = "yellow";
    else if (score > 0.4) color = "orange";
    else color = "red";
  }

  return (
    <Card key={doc.id} className="flex flex-col overflow-x-auto">
      <div className="flex gap-1 min-w-fit">
        <Link
          className="text-2xl text-[#006bb8] hover:underline font-medium"
          href={`/docs/${doc.id}`}
        >
          {doc.name}
        </Link>
        <LinkExternalIcon className="mt-auto mb-2" size={13} />
        {score !== undefined && (
          <div
            className={`my-auto ml-auto border-2 border-${color}-500 bg-${color}-50 w-10 h-10 rounded-full flex`}
          >
            <p className="m-auto font-mono">{(score * 100).toFixed()}</p>
          </div>
        )}
      </div>
      <p className="font-medium">{doc.file}</p>
      {doc.summary !== undefined && <p>{doc.summary}</p>}
    </Card>
  );
}
