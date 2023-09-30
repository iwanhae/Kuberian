import Card from "@/components/card";
import Header from "./header";

export const runtime = "edge";

export default async function Page(): Promise<JSX.Element> {
  const samples = [
    "checks if the system supports IPVS.",
    "authenticates the user request with certification.",
    "retrieves actual usage of CPU from container.",
    "checks available memory for each NUMA nodes.",
    "handles OOM events from the system.",
    "authenticates HTTP requests with x509 certificates.",
    "checks if the node has enough memory to deploy pod.",
    "generates service account token for pod.",
    "pulls an container image with image pull secret.",
    "registers a GPU device to kubelet",
    "creates iptable rules for network policy",
  ];

  return (
    <>
      <Header />
      <main className="flex flex-col">
        <div className="max-w-3xl w-full m-auto py-8 gap-4">
          <Card className="flex flex-col gap-2">
            <p className="text-3xl font-semibold">{`👋 Hi! I'm Kuberian!`} </p>
            <hr className="my-4 h-0.5 border-t-0 bg-neutral-100 opacity-100" />
            <p className="text-xl">
              {`Try `}
              <b>{`find functions that`}</b>
            </p>
            <ul className="text-xl list-disc list-inside">
              {samples
                .map((value) => ({ value, sort: Math.random() }))
                .sort((a, b) => a.sort - b.sort)
                .map(({ value }) => value)
                .slice(0, 4)
                .sort((a, b) => (a.length < b.length ? -1 : 1))
                .map((v, i) => {
                  return (
                    <li key={i}>
                      <a
                        className="text-[#006bb8] hover:underline"
                        href={`/q/${v}`}
                      >
                        {v}
                      </a>
                    </li>
                  );
                })}
            </ul>
            <p className="text-xl">
              <b>{`from Kubernetes `}</b> with Kuberian.
            </p>
            <hr className="w-1/2 m-auto h-0.5 border-t-0 bg-neutral-100 opacity-100" />
            <p className="text-xl">
              <b>Kuberian</b> is a Kubernetes code search service designed for
              DevOps Engineers or SREs seeking detailed information about
              Kubernetes. Just describe the functions you want to explore in the
              search box using natural language.
            </p>
            <p className="text-xl">
              With the power of the{" "}
              <a
                href="https://huggingface.co/TheBloke/Llama-2-7B-GGML"
                className="hover:underline text-blue-700"
              >
                Llama 2 7B
              </a>{" "}
              and{" "}
              <a
                href="https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2"
                className="hover:underline text-blue-700"
              >
                all-MiniLM-L6-v2
              </a>
              , you will find whatever you want to know much faster. 🚀🚀
            </p>
            <p className="text-xl">{"🤞 🍀 🌊 🔎 ⎈"}</p>
          </Card>
        </div>
      </main>
    </>
  );
}
