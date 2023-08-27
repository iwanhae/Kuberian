import React from "react";

export default function Card(props: {
  children: React.ReactNode;
}): JSX.Element {
  return (
    <section className="p-4 shadow-md border-[1px] rounded-md flex flex-col gap-1">
      {props.children}
    </section>
  );
}
