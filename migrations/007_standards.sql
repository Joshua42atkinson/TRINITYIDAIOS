-- 007_standards.sql — Academic standards database (NGSS, Common Core)
-- Supports align_standards tool and /api/standards/* endpoints

CREATE TABLE IF NOT EXISTS trinity_standards (
    id TEXT PRIMARY KEY,
    framework TEXT NOT NULL,
    subject TEXT NOT NULL,
    grade_band TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT '',
    code TEXT NOT NULL,
    description TEXT NOT NULL,
    performance_expectations TEXT DEFAULT '[]',
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_standards_framework
    ON trinity_standards (framework, subject, grade_band);

CREATE INDEX IF NOT EXISTS idx_standards_code
    ON trinity_standards (code);

-- NGSS Science Standards — selected high-frequency standards
INSERT OR IGNORE INTO trinity_standards (id, framework, subject, grade_band, category, code, description) VALUES
('ngss-3-5-ets1-1', 'NGSS', 'Science', '3-5', 'Engineering Design', '3-5-ETS1-1', 'Define a simple design problem reflecting a need or a want that includes specified criteria for success and constraints on materials, time, or cost.'),
('ngss-3-5-ets1-2', 'NGSS', 'Science', '3-5', 'Engineering Design', '3-5-ETS1-2', 'Generate and compare multiple possible solutions to a problem based on how well each is likely to meet the criteria and constraints of the problem.'),
('ngss-3-5-ets1-3', 'NGSS', 'Science', '3-5', 'Engineering Design', '3-5-ETS1-3', 'Plan and carry out fair tests in which variables are controlled and failure points are considered to identify aspects of a model or prototype that can be improved.'),
('ngss-ms-ets1-1', 'NGSS', 'Science', '6-8', 'Engineering Design', 'MS-ETS1-1', 'Define the criteria and constraints of a design problem with sufficient precision to ensure a successful solution, taking into account relevant scientific principles and potential impacts on people and the natural environment.'),
('ngss-ms-ets1-2', 'NGSS', 'Science', '6-8', 'Engineering Design', 'MS-ETS1-2', 'Evaluate competing design solutions using a systematic process to determine how well they meet the criteria and constraints of the problem.'),
('ngss-ms-ets1-3', 'NGSS', 'Science', '6-8', 'Engineering Design', 'MS-ETS1-3', 'Analyze data from tests to determine similarities and differences among several design solutions to identify the best characteristics of each that can be combined into a new solution.'),
('ngss-ms-ets1-4', 'NGSS', 'Science', '6-8', 'Engineering Design', 'MS-ETS1-4', 'Develop a model to generate data for iterative testing and modification of a proposed object, tool, or process such that an optimal design can be achieved.'),
('ngss-ms-ps2-2', 'NGSS', 'Science', '6-8', 'Physical Science', 'MS-PS2-2', 'Plan an investigation to provide evidence that the change in an object motion depends on the sum of the forces on the object and the mass of the object.'),
('ngss-ms-ps3-1', 'NGSS', 'Science', '6-8', 'Physical Science', 'MS-PS3-1', 'Construct and interpret graphical displays of data to describe the relationships of kinetic energy to the mass of an object and to the speed of an object.'),
('ngss-ms-ls1-1', 'NGSS', 'Science', '6-8', 'Life Science', 'MS-LS1-1', 'Conduct an investigation to provide evidence that living things are made of cells; either one cell or many different numbers and types of cells.'),
('ngss-ms-ess2-1', 'NGSS', 'Science', '6-8', 'Earth Science', 'MS-ESS2-1', 'Develop a model to describe the cycling of Earth materials and the flow of energy that drives this process.'),
('ngss-hs-ets1-1', 'NGSS', 'Science', '9-12', 'Engineering Design', 'HS-ETS1-1', 'Analyze a major global challenge to specify qualitative and quantitative criteria and constraints for solutions that account for societal needs and wants.'),
('ngss-hs-ets1-2', 'NGSS', 'Science', '9-12', 'Engineering Design', 'HS-ETS1-2', 'Design a solution to a complex real-world problem by breaking it down into smaller, more manageable problems that can be solved through engineering.'),
('ngss-hs-ets1-3', 'NGSS', 'Science', '9-12', 'Engineering Design', 'HS-ETS1-3', 'Evaluate a solution to a complex real-world problem based on prioritized criteria and trade-offs that account for a range of constraints.'),
('ngss-hs-ps1-1', 'NGSS', 'Science', '9-12', 'Physical Science', 'HS-PS1-1', 'Use the periodic table as a model to predict the relative properties of elements based on the patterns of electrons in the outermost energy level of atoms.'),
('ngss-hs-ps2-1', 'NGSS', 'Science', '9-12', 'Physical Science', 'HS-PS2-1', 'Analyze data to support the claim that Newtons second law of motion describes the mathematical relationship among the net force on a macroscopic object, its mass, and its acceleration.'),
('ngss-hs-ls1-1', 'NGSS', 'Science', '9-12', 'Life Science', 'HS-LS1-1', 'Construct an explanation based on evidence for how the sequence of DNA determines the structure of proteins which carry out the essential functions of life.'),
('ngss-hs-ess3-1', 'NGSS', 'Science', '9-12', 'Earth Science', 'HS-ESS3-1', 'Construct an explanation based on evidence for how the availability of natural resources, occurrence of natural hazards, and changes in climate have influenced human activity.'),
('ngss-k-2-ets1-1', 'NGSS', 'Science', 'K-2', 'Engineering Design', 'K-2-ETS1-1', 'Ask questions, make observations, and gather information about a situation people want to change to define a simple problem that can be solved through the development of a new or improved object or tool.'),
('ngss-k-2-ets1-2', 'NGSS', 'Science', 'K-2', 'Engineering Design', 'K-2-ETS1-2', 'Develop a simple sketch, drawing, or physical model to illustrate how the shape of an object helps it function as needed to solve a given problem.'),
('ngss-4-ps4-1', 'NGSS', 'Science', '4', 'Physical Science', '4-PS4-1', 'Develop a model of waves to describe patterns in terms of amplitude and wavelength and that waves can cause objects to move.'),
('ngss-4-ps3-2', 'NGSS', 'Science', '4', 'Physical Science', '4-PS3-2', 'Make observations to provide evidence that energy can be transferred from place to place by sound, light, heat, and electric currents.'),
('ngss-5-ess1-2', 'NGSS', 'Science', '5', 'Earth Science', '5-ESS1-2', 'Represent data in graphical displays to reveal patterns of daily changes in length and direction of shadows, day and night, and the seasonal appearance of some stars in the night sky.');

-- Common Core Math Standards — selected high-frequency standards
INSERT OR IGNORE INTO trinity_standards (id, framework, subject, grade_band, category, code, description) VALUES
('cc-math-3-oa-3', 'Common Core', 'Math', '3', 'Operations & Algebraic Thinking', '3.OA.A.3', 'Use multiplication and division within 100 to solve word problems in situations involving equal groups, arrays, and measurement quantities.'),
('cc-math-3-nf-1', 'Common Core', 'Math', '3', 'Number & Operations—Fractions', '3.NF.A.1', 'Understand a fraction 1/b as the quantity formed by 1 part when a whole is partitioned into b equal parts.'),
('cc-math-4-nbt-4', 'Common Core', 'Math', '4', 'Number & Operations in Base Ten', '4.NBT.B.4', 'Fluently add and subtract multi-digit whole numbers using the standard algorithm.'),
('cc-math-4-nf-3', 'Common Core', 'Math', '4', 'Number & Operations—Fractions', '4.NF.B.3', 'Understand a fraction a/b with a > 1 as a sum of fractions 1/b.'),
('cc-math-5-nbt-7', 'Common Core', 'Math', '5', 'Number & Operations in Base Ten', '5.NBT.B.7', 'Add, subtract, multiply, and divide decimals to hundredths, using concrete models or drawings and strategies based on place value.'),
('cc-math-5-md-1', 'Common Core', 'Math', '5', 'Measurement & Data', '5.MD.A.1', 'Convert among different-sized standard measurement units within a given measurement system.'),
('cc-math-6-ee-2', 'Common Core', 'Math', '6', 'Expressions & Equations', '6.EE.A.2', 'Write, read, and evaluate expressions in which letters stand for numbers.'),
('cc-math-6-rp-3', 'Common Core', 'Math', '6', 'Ratios & Proportional Relationships', '6.RP.A.3', 'Use ratio and rate reasoning to solve real-world and mathematical problems.'),
('cc-math-7-ee-4', 'Common Core', 'Math', '7', 'Expressions & Equations', '7.EE.B.4', 'Use variables to represent quantities in a real-world or mathematical problem, and construct simple equations and inequalities to solve problems.'),
('cc-math-7-rp-2', 'Common Core', 'Math', '7', 'Ratios & Proportional Relationships', '7.RP.A.2', 'Recognize and represent proportional relationships between quantities.'),
('cc-math-8-ee-5', 'Common Core', 'Math', '8', 'Expressions & Equations', '8.EE.B.5', 'Graph proportional relationships, interpreting the unit rate as the slope of the graph.'),
('cc-math-8-f-3', 'Common Core', 'Math', '8', 'Functions', '8.F.B.3', 'Interpret the equation y = mx + b as defining a linear function, whose graph is a straight line.'),
('cc-math-hs-a-rei-3', 'Common Core', 'Math', '9-12', 'Algebra', 'HSA-REI.B.3', 'Solve linear equations and inequalities in one variable, including equations with coefficients represented by letters.'),
('cc-math-hs-a-ced-1', 'Common Core', 'Math', '9-12', 'Algebra', 'HSA-CED.A.1', 'Create equations and inequalities in one variable and use them to solve problems.'),
('cc-math-hs-f-if-1', 'Common Core', 'Math', '9-12', 'Functions', 'HSF-IF.A.1', 'Understand that a function from one set (called the domain) to another set (called the range) assigns to each element of the domain exactly one element of the range.'),
('cc-math-hs-g-co-10', 'Common Core', 'Math', '9-12', 'Geometry', 'HSG-CO.C.10', 'Prove theorems about triangles.'),
('cc-math-hs-s-id-2', 'Common Core', 'Math', '9-12', 'Statistics & Probability', 'HSS-ID.A.2', 'Use statistics appropriate to the shape of the data distribution to compare center and spread of two or more different data sets.');

-- Common Core ELA Standards — selected high-frequency standards
INSERT OR IGNORE INTO trinity_standards (id, framework, subject, grade_band, category, code, description) VALUES
('cc-ela-4-ri-1', 'Common Core', 'ELA', '4', 'Reading Informational', 'RI.4.1', 'Refer to details and examples in a text when explaining what the text says explicitly and when drawing inferences from the text.'),
('cc-ela-4-ri-3', 'Common Core', 'ELA', '4', 'Reading Informational', 'RI.4.3', 'Explain events, procedures, ideas, or concepts in a historical, scientific, or technical text, including what happened and why.'),
('cc-ela-5-ri-2', 'Common Core', 'ELA', '5', 'Reading Informational', 'RI.5.2', 'Determine two or more main ideas of a text and explain how they are supported by key details.'),
('cc-ela-6-ri-1', 'Common Core', 'ELA', '6', 'Reading Informational', 'RI.6.1', 'Cite textual evidence to support analysis of what the text says explicitly as well as inferences drawn from the text.'),
('cc-ela-6-ri-4', 'Common Core', 'ELA', '6', 'Reading Informational', 'RI.6.4', 'Determine the meaning of words and phrases as they are used in a text, including figurative, connotative, and technical meanings.'),
('cc-ela-7-ri-2', 'Common Core', 'ELA', '7', 'Reading Informational', 'RI.7.2', 'Determine two or more central ideas in a text and analyze their development over the course of the text.'),
('cc-ela-8-ri-3', 'Common Core', 'ELA', '8', 'Reading Informational', 'RI.8.3', 'Analyze how a text makes connections among and distinctions between individuals, ideas, or events.'),
('cc-ela-9-10-ri-1', 'Common Core', 'ELA', '9-10', 'Reading Informational', 'RI.9-10.1', 'Cite strong and thorough textual evidence to support analysis of what the text says explicitly.'),
('cc-ela-9-10-ri-2', 'Common Core', 'ELA', '9-10', 'Reading Informational', 'RI.9-10.2', 'Determine a central idea of a text and analyze its development over the course of the text.'),
('cc-ela-11-12-ri-2', 'Common Core', 'ELA', '11-12', 'Reading Informational', 'RI.11-12.2', 'Determine two or more central ideas of a text and analyze their development over the course of the text.'),
('cc-ela-4-w-2', 'Common Core', 'ELA', '4', 'Writing', 'W.4.2', 'Write informative/explanatory texts to examine a topic and convey ideas and information clearly.'),
('cc-ela-6-w-1', 'Common Core', 'ELA', '6', 'Writing', 'W.6.1', 'Write arguments to support claims with clear reasons and relevant evidence.'),
('cc-ela-7-w-3', 'Common Core', 'ELA', '7', 'Writing', 'W.7.3', 'Write narratives to develop real or imagined experiences or events using effective technique, relevant descriptive details, and well-structured event sequences.'),
('cc-ela-8-sl-1', 'Common Core', 'ELA', '8', 'Speaking & Listening', 'SL.8.1', 'Engage effectively in a range of collaborative discussions with diverse partners on grade 8 topics, texts, and issues.'),
('cc-ela-9-10-sl-4', 'Common Core', 'ELA', '9-10', 'Speaking & Listening', 'SL.9-10.4', 'Present claims and findings, emphasizing salient points in a focused, coherent manner with relevant evidence, sound valid reasoning, and well-chosen details.');
