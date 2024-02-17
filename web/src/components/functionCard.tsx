"use client";
import "./github-light.css";
import { type Doc } from "@/types";
import Card from "./card";
import Link from "next/link";
import { LinkExternalIcon } from "@primer/octicons-react";
import { useEffect } from "react";
import hljs from "highlight.js";

export default function FunctionCard({
  doc,
  score,
}: {
  doc: Doc;
  score?: number;
}): JSX.Element {
  useEffect(() => {
    hljs.highlightAll();
  }, []);

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
          href={
            doc.extra == null
              ? `/docs/${doc.id}`
              : `https://github.com/kubernetes/kubernetes/blob/v1.27.4/${doc.file}#L${doc.line.start}-L${doc.line.end}`
          }
        >
          {doc.name}
        </Link>
        <div className="relative">
          <LinkExternalIcon className="absolute top-3.5" size={13} />
        </div>
        {score !== undefined && (
          <div
            className={`my-auto ml-auto border-2 border-${color}-500 bg-${color}-50 w-10 h-10 rounded-full flex`}
          >
            <p className="m-auto font-mono">{(score * 100).toFixed()}</p>
          </div>
        )}
      </div>
      <p className="font-medium">{doc.file}</p>
      {doc.summary != null && <p>{doc.summary}</p>}
      {doc.extra != null && (
        <>
          <pre className="overflow-auto border rounded-lg text-sm my-4">
            <code className="language-go">{doc.extra.code}</code>
          </pre>
          <section>
            <h5 className="font-bold">Background</h5>
            <p className="whitespace-pre-wrap indent-2">
              {doc.extra.background}
            </p>
          </section>
          <section>
            <h5 className="font-bold">Anaylsis</h5>
            <p className="whitespace-pre-wrap indent-2">{doc.extra.analysis}</p>
          </section>
          <section>
            <h5 className="font-bold">Purpose</h5>
            <p className="whitespace-pre-wrap indent-2">{doc.extra.purpose}</p>
          </section>
          <hr className="my-4 h-0.5 border-t-0 bg-neutral-100 opacity-100" />
          <section>
            <h5 className="font-bold">Comment</h5>
            <p className="whitespace-pre-wrap indent-2">{doc.extra.comment}</p>
          </section>
          <hr className="my-4 h-0.5 border-t-0 bg-neutral-100 opacity-100" />
          <section>
            <h5 className="font-bold">Opinion</h5>
            <p className="whitespace-pre-wrap indent-2">{doc.extra.opinion}</p>
          </section>
        </>
      )}
    </Card>
  );
}
