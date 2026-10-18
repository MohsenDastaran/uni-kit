//! `questionnaire` — a multi-step form with choices and free text.
import { signal } from "nsh";
import { Questionnaire } from "../components/questionnaire";
import { Caption, ExampleSection, Page, Status } from "./ui";

const questions = [
  { id: "role", title: "What is your role?", kind: "choice" as const, options: [{ value: "engineer", label: "Engineer" }, { value: "designer", label: "Designer" }, { value: "writer", label: "Writer" }] },
  { id: "goal", title: "What are you building?", kind: "text" as const, required: true },
  { id: "size", title: "How large is the team?", kind: "choice" as const, options: [{ value: "small", label: "1–10" }, { value: "medium", label: "11–50" }, { value: "large", label: "51+" }] },
];

export function Demo() {
  const answers = signal<Record<string, string>>({});
  // section: Usage
  const usage = <Questionnaire questions={questions} answers={answers()} onChange={(next) => answers.set(next)} onComplete={() => {}} />;
  // section: Choices
  const choices = <Questionnaire questions={[questions[0]]} />;
  // section: Validation
  const validation = <Questionnaire questions={[questions[1]]} />;
  // section: Navigation and submission
  const navigation = <Status>{Object.keys(answers()).length} answered</Status>;
  // section: end
  return (
    <Page>
      <Caption>The questionnaire owns the step and the answers; the questions are the application's model.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="Choices">{choices}</ExampleSection>
      <ExampleSection index={2} title="Validation">{validation}</ExampleSection>
      <ExampleSection index={3} title="Navigation and submission">{navigation}</ExampleSection>
    </Page>
  );
}
