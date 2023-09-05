export interface SearchResponse {
  query: string;
  docs: Doc[];
  results: Result[];
}

export type DocResponse = Doc;

export interface Doc {
  id: number;
  name: string;
  signature: string;
  file: string;
  line: Line;
  summary?: string;
  extra?: DocExtra;
}

export interface DocExtra {
  code?: string;
  background?: string;
  analysis?: string;
  purpose?: string;
  comment?: string;
  opinion?: string;
}

interface Line {
  start: number;
  end: number;
}

interface Result {
  id: number;
  score: number;
}
