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
  const discovery = signal<Record<string, string>>({});
  // section: How did you first hear about us?
  const discoveryCard = (
    <Questionnaire
      questions={[
        {
          id: "discovery",
          title: "How did you first hear about us?",
          description: "Pick the closest match, or write in what actually happened.",
          kind: "choice",
          options: [
            { value: "colleague", label: "A colleague mentioned it" },
            { value: "conference", label: "A conference talk" },
            { value: "search", label: "Search" },
            { value: "newsletter", label: "A newsletter" },
          ],
          other: { label: "Somewhere else", placeholder: "Tell us where" },
        },
        { id: "role", title: "What best describes your role?", kind: "choice", options: [{ value: "engineer", label: "Engineer" }, { value: "designer", label: "Designer" }, { value: "founder", label: "Founder" }] },
        { id: "notes", title: "Anything else we should know?", kind: "text" },
      ]}
      answers={discovery()}
      onChange={(next) => discovery.set(next)}
      onComplete={() => {}}
      style={{ width: 440, maxWidth: "100%" }}
    />
  );

  // section: Usage
  const usage = <Questionnaire questions={questions} answers={answers()} onChange={(next) => answers.set(next)} onComplete={() => {}} style={{ width: 460, maxWidth: "100%" }} />;
  // section: Choices
  const choices = <Questionnaire questions={[questions[0]]} style={{ width: 440, maxWidth: "100%" }} />;
  // section: Validation
  const validation = <Questionnaire questions={[questions[1]]} style={{ width: 440, maxWidth: "100%" }} />;
  // section: Navigation and submission
  const navigation = <Status>{Object.keys(answers()).length} answered</Status>;
  // section: end
  return (
    <Page>
      <Caption>The questionnaire owns the step and the answers; the questions are the application's model.</Caption>
      <ExampleSection index={0} title="Usage">{usage}</ExampleSection>
      <ExampleSection index={1} title="How did you first hear about us?">{discoveryCard}</ExampleSection>
      <ExampleSection index={2} title="Choices">{choices}</ExampleSection>
      <ExampleSection index={3} title="Validation">{validation}</ExampleSection>
      <ExampleSection index={4} title="Navigation and submission">{navigation}</ExampleSection>
    </Page>
  );
}
