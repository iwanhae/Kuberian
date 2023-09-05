import React from "react";

export default function Card(props: {
  children: React.ReactNode;
  className?: string;
}): JSX.Element {
  return (
    <section
      className={`p-4 shadow-md border-[1px] rounded-md ${props.className}`}
    >
      {props.children}
    </section>
  );
}
