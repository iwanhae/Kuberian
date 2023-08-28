import Link from "next/link";
import Card from "./card";
import Header from "./header";
import { SERVER_URL } from "@/config";

export const runtime = "edge";

async function getData(): Promise<{
  total: number;
  analyzed: number;
  samples: string[];
}> {
  const res = await fetch(SERVER_URL(), {
    next: { revalidate: 5 },
  });

  if (!res.ok) {
    throw new Error("Failed to fetch data");
  }

  return await res.json();
}

export default async function Page(): Promise<JSX.Element> {
  const data = await getData();

  return (
    <>
      <Header />
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 gap-4">
          <Card>
            <p className="text-3xl font-semibold">{`👋 Hi! I'm Kuberian!`} </p>
            <p className="text-xl">{`If you are`}</p>
            <ol className="list-disc list-outside pl-5 text-xl">
              <li>
                Navigating the complex landscape of Kubernetes and encountering
                undocumented features? 😵‍💫
              </li>
              <li>
                Seeking clarity on intricate functionalities buried within the
                source code? 🤯
              </li>
            </ol>
            <p className="text-xl">{`I'm here to assist you! Here's how to harness the power of Kuberian: 🚀`}</p>
            <p className="text-xl">
              {`Simply describe your query in natural language, like `}
              <b>{`"find functions that`}</b>
            </p>
            <div className="my-4 shadow-md flex flex-col text-xl font-medium">
              {data.samples
                .sort((a, b) => (a.length < b.length ? -1 : 1))
                .slice(0, 5)
                .map((v, i) => {
                  return (
                    <Link
                      key={i}
                      className="p-2 border first:rounded-t-lg last:rounded-b-lg odd:bg-slate-100 hover:bg-sky-200 cursor-pointer transition-all text-ellipsis whitespace-nowrap overflow-hidden"
                      prefetch={false}
                      href={`/q/${v}`}
                    >
                      {v}
                    </Link>
                  );
                })}
            </div>
            <p className="text-xl">
              <b>{`from Kubernetes." `}</b>
              {`Our intelligent search will sift through the Kubernetes source
              code and pinpoint the exact functions you need.`}
            </p>
            <p className="text-xl">
              Say goodbye to guesswork and hours of sifting through code!
              Welcome to a hassle-free journey of understanding Kubernetes
              intricacies. 🌟
            </p>
            <p></p>
          </Card>
        </div>
      </main>
    </>
  );
}
