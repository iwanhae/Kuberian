export const runtime = 'edge'

async function getData (): Promise<{
  total: number
  analyzed: number
  samples: string[]
}> {
  const res = await fetch('https://kuberian-thoe7ww5ya-an.a.run.app/', { next: { revalidate: 5 } })
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
