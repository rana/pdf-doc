use pdf_doc::*;

fn main() {
    // Create a document template.
    let spc: f32 = 0.6;
    let spc_aft: f32 = 2.0 * spc;
    let mut tmpl = new_ansi_letter()
        .set_spc_lne(LineSpace::Custom(1.65))
        .set_spc_par_aft(LineSpace::Custom(spc));
    tmpl.add_par(
        par(S1)
            .set_has_ind(Some(false))
            .set_spc_aft(Some(LineSpace::Custom(spc_aft))),
    );
    tmpl.add_par(par(S2));
    tmpl.add_par(par(S3));
    tmpl.add_par(par(S4));
    tmpl.add_par(par(S5));
    tmpl.add_par(par(S6).set_spc_aft(Some(LineSpace::Custom(spc_aft))));
    tmpl.add_par(
        par(S7)
            .set_has_ind(Some(false))
            .set_fnt_sty(Some(Style::Italic)),
    );

    // Create a letter document from the template.
    let mut ltr = tmpl.clone_clear();

    // Iterate through each name.
    let names = ["Albert Einstein", "Richard Feynman", "Paul Dirac"];
    for name in names {
        // Clone template.
        let mut cur_doc = tmpl.clone();
        // Replace placeholder text with actual name.
        cur_doc.replace_par_at(0, "{{name}}", name);
        // Copy paragraphs to destination letter.
        ltr.copy_pars(cur_doc.clone());
        // Add a page break.
        ltr.add_pag_brk();
    }

    // Save letter in JSON format.
    ltr.save_json("doc").unwrap();
    let ltr2 = ltr.read_json("doc").unwrap();

    // Save letter in PDF format.
    ltr2.save_pdf("doc").unwrap();
}

const S1: &str = "Dear {{name}},";
const S2: &str = "Quantum Chromodynamics (QCD) is the theory describing the strong force, one of nature's fundamental interactions. Quarks and gluons are the particles governed by QCD. Unlike the electromagnetic force, which is described by Quantum Electrodynamics (QED), the strong force is more complex.";
const S3: &str = "Gluons, the force carriers of the strong force, carry color charge. This property, combined with asymptotic freedom (quarks behave as free particles at high energies) and color confinement (quarks are always bound together), makes QCD distinct from QED. QCD is essential for understanding matter, nuclear physics, and the early universe.";
const S4: &str = "QCD calculations are challenging due to the complex nature of gluon interactions. Unlike QED, where calculations are often straightforward, QCD requires advanced techniques like lattice QCD for low-energy calculations. Despite these challenges, significant progress has been made in understanding hadrons.";
const S5: &str = "The strong force played a crucial role in the early universe. As the universe cooled, quarks and gluons combined to form hadrons. The properties of this quark-gluon plasma are still being investigated. Understanding QCD in extreme conditions is vital for our knowledge of the universe's evolution.";
const S6: &str = "QCD is a cornerstone of the Standard Model of particle physics and continues to be an active area of research. Many fundamental questions about the strong force remain unanswered, such as the precise mechanism of confinement and the detailed structure of hadrons.";
const S7: &str = "-Gemini AI";
