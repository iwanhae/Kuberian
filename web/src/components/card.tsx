import React from "react";

export default function Card(props: {
  children: React.ReactNode;
  className?: string;
  onClick?: () => void;
}): JSX.Element {
  return (
    <section
      className={`p-4 shadow-md border-[1px] rounded-md ${props.className}`}
      onClick={props.onClick}
    >
      {props.children}
    </section>
  );
}
