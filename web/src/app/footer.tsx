"use client";
import Giscus from "@giscus/react";

export default function Footer(): JSX.Element {
  return (
    <div className="max-w-3xl w-full m-auto py-8 gap-4">
      <Giscus
        id="comments"
        repo="iwanhae/Kuberian"
        repoId="R_kgDOKDGVSw"
        mapping="number"
        term="1"
        reactionsEnabled="1"
        emitMetadata="1"
        inputPosition="top"
        theme="light"
        lang="en"
        loading="eager"
      />
    </div>
  );
}
