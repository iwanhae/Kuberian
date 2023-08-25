export const runtime = 'edge'

async function getData (): Promise<{
  total: number
  analyzed: number
  samples: string[]
}> {
  const res = await fetch('https://kuberian-thoe7ww5ya-an.a.run.app/', {
    next: { revalidate: 5 }
  })
  // The return value is *not* serialized
  // You can return Date, Map, Set, etc.

  if (!res.ok) {
    // This will activate the closest `error.js` Error Boundary
    throw new Error('Failed to fetch data')
  }

  return await res.json()
}

export default async function Page (): Promise<JSX.Element> {
  const data = await getData()

  return (
    <main>
      <header className="h-72 bg-slate-100 border-b-[1px]">
        <div className="max-w-3xl h-full m-auto pb-6 flex flex-col justify-end gap-2">
          <h3>Kubernetes Librarian : Kuberian</h3>
          <h1 className="text-4xl font-bold">Find functions that</h1>
          <div></div>
          <div className="w-full">
            <input type="text" className="w-full text-4xl"></input>
          </div>
          <div></div>
          <h1 className="text-4xl font-bold flex gap-2">
            <p>from</p>
            <a
              className="text-[#006bb8] font-medium flex gap-2"
              href="https://github.com/kubernetes/kubernetes/tree/v1.27.4"
              target="_blank"
            >
              <p>Kubernetes</p>
              <svg
                className="m-auto"
                xmlns="http://www.w3.org/2000/svg"
                width="14"
                height="14"
                viewBox="0 0 16 16"
                role="img"
                data-icon-type="popout"
                data-is-loaded="true"
                aria-label="External link"
              >
                <path d="M13 8.5a.5.5 0 1 1 1 0V12a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h3.5a.5.5 0 0 1 0 1H4a1 1 0 0 0-1 1v8a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1V8.5Zm-5.12.339a.5.5 0 1 1-.706-.707L13.305 2H10.5a.5.5 0 1 1 0-1H14a1 1 0 0 1 1 1v3.5a.5.5 0 1 1-1 0V2.72L7.88 8.838Z"></path>
              </svg>
            </a>
          </h1>
        </div>
      </header>
      <h5>Total: {data.total}</h5>
      <h5>Analyzed: {data.analyzed}</h5>
      <ol>
        {data.samples.map((v, i) => {
          return <li key={i}>{v}</li>
        })}
      </ol>
    </main>
  )
}
