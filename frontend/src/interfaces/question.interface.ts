import Answer from './answer.interface'

export default interface Question {
  id: number
  question: string
  answers: Answer[]
}
