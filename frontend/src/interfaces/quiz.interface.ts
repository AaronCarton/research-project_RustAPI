import Question from './question.interface'

export default interface Quiz {
  id: number
  name: string
  description: string
  questions: Question[]
  score: number
}
