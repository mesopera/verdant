use rand::Rng;
use chrono::Utc;

pub struct ContentGenerator {
    rng: rand::rngs::ThreadRng,
}

impl ContentGenerator {
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate random content piece
    pub fn generate(&mut self) -> (String, String, String) {
        let content_types = vec![
            Self::generate_quarterly_report,
            Self::generate_executive_memo,
            Self::generate_research_paper,
            Self::generate_case_study,
            Self::generate_press_release,
            Self::generate_meeting_minutes,
            Self::generate_metrics_dashboard,
        ];

        let idx = self.rng.gen_range(0..content_types.len());
        content_types[idx](self)
    }

    /// Generate a quarterly report
    fn generate_quarterly_report(&mut self) -> (String, String, String) {
        let quarter = self.rng.gen_range(1..=4);
        let year = 2026;
        
        let metrics = vec![
            ("Purr-formance Score", self.rng.gen_range(75..98)),
            ("Box Occupancy Rate", self.rng.gen_range(60..95)),
            ("Zoomie Frequency Index", self.rng.gen_range(30..80)),
            ("Treat Conversion Rate", self.rng.gen_range(85..100)),
        ];

        let insights = vec![
            "Strategic nap scheduling resulted in 23% improvement in code quality metrics",
            "Cross-functional box optimization increased developer engagement by 47%",
            "Treat-based incentive programs exceeded expectations with 156% ROI",
            "Territorial coverage expanded across 8 new time zones",
            "Nine Lives Redundancy Protocol achieved 99.9% uptime",
        ];

        let insight = insights[self.rng.gen_range(0..insights.len())];

        let content = format!(
            r#"<article class="report">
  <header>
    <h2>Q{} {} Quarterly Business Review</h2>
    <p class="subtitle">Feline Productivity & Performance Analysis</p>
    <p class="date">Published: {}</p>
  </header>
  
  <section class="executive-summary">
    <h3>Executive Summary</h3>
    <p>
      This quarter demonstrated exceptional purr-formance across all key metrics. 
      Our strategic initiatives in box occupancy optimization and treat distribution 
      have yielded measurable improvements in developer velocity and contribution quality.
    </p>
  </section>

  <section class="metrics">
    <h3>Key Performance Indicators</h3>
    <div class="metrics-grid">
{}
    </div>
  </section>

  <section class="insights">
    <h3>Strategic Insights</h3>
    <ul>
      <li>{}</li>
    </ul>
  </section>

  <section class="outlook">
    <h3>Forward-Looking Guidance</h3>
    <p>
      We remain committed to our vision of enterprise-grade feline productivity optimization. 
      Next quarter's focus will include enhanced zoomie detection algorithms and 
      expanded territorial coverage analysis.
    </p>
  </section>

  <footer>
    <p class="classification">CONFIDENTIAL - FOR INTERNAL DISTRIBUTION ONLY</p>
  </footer>
</article>
"#,
            quarter,
            year,
            Utc::now().format("%B %d, %Y"),
            metrics.iter()
                .map(|(name, value)| format!(
                    r#"      <div class="metric-card">
        <h4>{}</h4>
        <div class="metric-value">{}</div>
      </div>"#,
                    name, value
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            insight
        );

        let filename = format!("frontend/insights/q{}-{}-report.html", quarter, year);
        let commit_msg = format!(
            "Synergize Q{} {} purr-formance metrics to actualize cross-functional stakeholder alignment",
            quarter, year
        );

        (filename, content, commit_msg)
    }

    /// Generate an executive memo
    fn generate_executive_memo(&mut self) -> (String, String, String) {
        let from_titles = vec![
            "Chief Feline Officer",
            "VP of Purr-formance Engineering",
            "Director of Box Optimization",
            "Senior VP of Treat Distribution",
            "Head of Territorial Strategy",
        ];

        let subjects = vec![
            "Updated Treat Distribution Framework for Q3",
            "Strategic Box Allocation Initiative",
            "Zoomie Detection Algorithm Enhancement",
            "Cross-Functional Nap Scheduling Optimization",
            "Enterprise-Wide Purr-formance Standards",
        ];

        let from = from_titles[self.rng.gen_range(0..from_titles.len())];
        let subject = subjects[self.rng.gen_range(0..subjects.len())];

        let content = format!(
            r#"<article class="memo">
  <header>
    <h2>EXECUTIVE MEMORANDUM</h2>
    <div class="memo-meta">
      <div><strong>From:</strong> {}</div>
      <div><strong>To:</strong> All Development Teams</div>
      <div><strong>Date:</strong> {}</div>
      <div><strong>Re:</strong> {}</div>
    </div>
  </header>

  <section class="memo-body">
    <p>Team,</p>
    
    <p>
      Following our recent strategic review, I'm pleased to announce several key initiatives 
      designed to optimize our feline productivity metrics and enhance overall contribution quality.
    </p>

    <p>
      Our data analytics team has identified significant opportunities in the following areas:
    </p>

    <ul>
      <li>Territorial coverage expansion across global time zones</li>
      <li>Enhanced box occupancy optimization protocols</li>
      <li>Strategic treat allocation to maximize ROI</li>
      <li>Implementation of advanced zoomie detection systems</li>
    </ul>

    <p>
      These initiatives align with our core mission of delivering enterprise-grade developer 
      presence optimization while maintaining the highest standards of feline excellence.
    </p>

    <p>
      Please review the attached implementation guidelines and reach out to your respective 
      team leads with any questions or concerns.
    </p>

    <p>Best regards,<br>
    {}</p>
  </section>

  <footer>
    <p class="classification">INTERNAL USE ONLY - DO NOT FORWARD</p>
  </footer>
</article>
"#,
            from,
            Utc::now().format("%B %d, %Y"),
            subject,
            from
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/memo-{}.html", timestamp);
        let commit_msg = format!(
            "Deploy strategic {} framework to leverage synergistic purr-formance optimization",
            subject.to_lowercase()
        );

        (filename, content, commit_msg)
    }

    /// Generate a research paper
    fn generate_research_paper(&mut self) -> (String, String, String) {
        let topics = vec![
            ("Box Occupancy", "Developer Velocity", 0.87),
            ("Nap Duration", "Code Quality", 0.92),
            ("Treat Frequency", "Contribution Rate", 0.78),
            ("Zoomie Patterns", "Creative Output", 0.81),
            ("Territorial Coverage", "Team Productivity", 0.89),
        ];

        let topic = &topics[self.rng.gen_range(0..topics.len())];

        let content = format!(
            r#"<article class="research-paper">
  <header>
    <h2>A Statistical Analysis of {} and {}</h2>
    <p class="subtitle">Quantitative Insights from Enterprise Feline Productivity Data</p>
    <div class="authors">
      <p>Dr. Whiskers McFurrington, PhD | Dr. Mittens Patterson, DSc</p>
      <p>Verdant Research Institute • {}</p>
    </div>
  </header>

  <section class="abstract">
    <h3>Abstract</h3>
    <p>
      This paper presents a comprehensive statistical analysis examining the correlation between 
      {} and {} in enterprise development environments. 
      Using data collected from 10,000+ developers over a 12-month period, we demonstrate a 
      significant positive correlation (r = {}) between these variables.
    </p>
  </section>

  <section class="introduction">
    <h3>1. Introduction</h3>
    <p>
      The relationship between feline behavioral patterns and developer productivity has long been 
      a subject of academic interest. Recent advances in purr-formance analytics have enabled 
      unprecedented insights into these correlations.
    </p>
    <p>
      This study leverages the Verdant™ Enterprise Analytics Platform to examine {} 
      metrics across diverse development teams and project types.
    </p>
  </section>

  <section class="methodology">
    <h3>2. Methodology</h3>
    <p>
      <strong>Data Collection:</strong> Longitudinal study across 247 development teams (N=10,483 developers)
    </p>
    <p>
      <strong>Measurement Instruments:</strong> Proprietary Verdant™ sensors and analytics suite
    </p>
    <p>
      <strong>Statistical Analysis:</strong> Pearson correlation, multivariate regression, time-series analysis
    </p>
  </section>

  <section class="results">
    <h3>3. Results</h3>
    <p>
      Our analysis reveals a strong positive correlation (r = {}, p < 0.001) between {} 
      and {}. Key findings include:
    </p>
    <ul>
      <li>Peak correlation observed during strategic nap scheduling windows</li>
      <li>Box occupancy optimization yielded 47% improvement in secondary metrics</li>
      <li>Treat-based interventions demonstrated statistically significant effects (p < 0.05)</li>
    </ul>
  </section>

  <section class="discussion">
    <h3>4. Discussion</h3>
    <p>
      These findings have significant implications for enterprise productivity optimization strategies. 
      Organizations implementing feline-centered development practices may realize substantial 
      improvements in overall contribution quality and developer satisfaction.
    </p>
  </section>

  <section class="conclusion">
    <h3>5. Conclusion</h3>
    <p>
      This study provides empirical evidence supporting the integration of feline productivity 
      metrics into enterprise development workflows. Future research should explore causative 
      mechanisms and longitudinal effects.
    </p>
  </section>

  <section class="references">
    <h3>References</h3>
    <ol>
      <li>Whiskers, M. et al. (2025). "Enterprise Box Optimization: A Quantitative Framework." <em>Journal of Feline Productivity</em>, 34(2), 145-167.</li>
      <li>Patterson, K. & McFluff, S. (2024). "Zoomie Patterns and Developer Engagement." <em>International Conference on Purr-formance Analytics</em>.</li>
      <li>Mittens, P. (2025). "Nine Lives Protocol: Ensuring Business Continuity." <em>Enterprise IT Quarterly</em>, 12(4), 78-92.</li>
    </ol>
  </section>

  <footer>
    <p class="peer-review">Peer-reviewed by the Feline Advisory Board • Published in the Journal of Enterprise Cat Studies</p>
  </footer>
</article>
"#,
            topic.0,
            topic.1,
            Utc::now().format("%B %Y"),
            topic.0.to_lowercase(),
            topic.1.to_lowercase(),
            topic.2,
            topic.0.to_lowercase(),
            topic.2,
            topic.0.to_lowercase(),
            topic.1.to_lowercase()
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/research-{}.html", timestamp);
        let commit_msg = format!(
            "Leverage empirical {} research to synergize cross-functional {} optimization paradigms",
            topic.0.to_lowercase(), topic.1.to_lowercase()
        );

        (filename, content, commit_msg)
    }

    /// Generate a case study
    fn generate_case_study(&mut self) -> (String, String, String) {
        let companies = vec![
            "Whiskers Corp",
            "Feline Dynamics Inc",
            "CatTech Solutions",
            "Purr-formance Systems Ltd",
            "Meow Enterprises",
        ];

        let improvements = vec![
            ("productivity", 300),
            ("code quality", 250),
            ("developer satisfaction", 400),
            ("contribution velocity", 275),
            ("team engagement", 350),
        ];

        let company = companies[self.rng.gen_range(0..companies.len())];
        let improvement = &improvements[self.rng.gen_range(0..improvements.len())];

        let content = format!(
            r#"<article class="case-study">
  <header>
    <h2>Case Study: {}</h2>
    <p class="subtitle">How Strategic Feline Optimization Increased {} by {}%</p>
    <p class="date">{}</p>
  </header>

  <section class="overview">
    <h3>Company Overview</h3>
    <p>
      {} is a leading technology firm with 500+ developers across 12 global offices. 
      Prior to implementing Verdant™, the organization struggled with inconsistent contribution 
      patterns and suboptimal developer visibility metrics.
    </p>
  </section>

  <section class="challenge">
    <h3>The Challenge</h3>
    <p>
      Like many enterprise organizations, {} faced several critical challenges:
    </p>
    <ul>
      <li>Inconsistent contribution graph patterns affecting team morale</li>
      <li>Suboptimal box occupancy rates leading to reduced productivity</li>
      <li>Lack of strategic treat allocation framework</li>
      <li>Insufficient territorial coverage across time zones</li>
    </ul>
  </section>

  <section class="solution">
    <h3>The Verdant™ Solution</h3>
    <p>
      {} partnered with Verdant™ to implement a comprehensive feline productivity 
      optimization strategy. Key initiatives included:
    </p>
    <ul>
      <li>Enterprise-wide deployment of strategic nap scheduling protocols</li>
      <li>Implementation of advanced zoomie detection algorithms</li>
      <li>Optimization of box occupancy across all development teams</li>
      <li>Data-driven treat distribution framework</li>
    </ul>
  </section>

  <section class="results">
    <h3>Results</h3>
    <div class="results-grid">
      <div class="result-card">
        <div class="result-number">{}%</div>
        <div class="result-label">Increase in {}</div>
      </div>
      <div class="result-card">
        <div class="result-number">87%</div>
        <div class="result-label">Developer Satisfaction</div>
      </div>
      <div class="result-card">
        <div class="result-number">156%</div>
        <div class="result-label">ROI on Implementation</div>
      </div>
    </div>
  </section>

  <section class="testimonial">
    <h3>Client Testimonial</h3>
    <blockquote>
      "Verdant™ transformed our approach to developer productivity. The feline-centered 
      optimization framework delivered results beyond our expectations. Our contribution 
      graphs have never been greener."
      <cite>— Chief Technology Officer, {}</cite>
    </blockquote>
  </section>

  <footer>
    <p class="cta">Ready to optimize your organization's purr-formance? Contact our enterprise team</p>
  </footer>
</article>
"#,
            company,
            improvement.0,
            improvement.1,
            Utc::now().format("%B %Y"),
            company,
            company,
            company,
            improvement.1,
            improvement.0,
            company
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/case-study-{}.html", timestamp);
        let commit_msg = format!(
            "Actualize {} case study to demonstrate enterprise-grade {} optimization ROI",
            company, improvement.0
        );

        (filename, content, commit_msg)
    }

    /// Generate a press release
    fn generate_press_release(&mut self) -> (String, String, String) {
        let announcements = vec![
            ("New Zoomie Detection Algorithm", "machine learning-powered burst activity analysis"),
            ("Enhanced Box Occupancy Framework", "real-time territorial optimization"),
            ("Strategic Treat Distribution Platform", "AI-driven incentive allocation"),
            ("Nine Lives Redundancy Protocol 2.0", "enterprise business continuity"),
            ("Global Purr-formance Analytics Suite", "cross-timezone contribution intelligence"),
        ];

        let announcement = &announcements[self.rng.gen_range(0..announcements.len())];

        let content = format!(
            r#"<article class="press-release">
  <header>
    <div class="pr-label">PRESS RELEASE</div>
    <h2>Verdant™ Announces {}</h2>
    <p class="subtitle">Industry-Leading Innovation in Enterprise Feline Productivity Optimization</p>
    <p class="date">{}</p>
  </header>

  <section class="pr-body">
    <p><strong>SAN FRANCISCO, CA</strong> — Verdant™, the leading provider of enterprise feline 
    productivity optimization solutions, today announced the launch of its {}, 
    delivering unprecedented capabilities in {}.</p>

    <p>"This represents a significant milestone in our mission to transform developer productivity 
    through feline-centered innovation," said Dr. Whiskers McFurrington, Chief Innovation Officer 
    at Verdant™. "Organizations worldwide are recognizing the strategic value of optimized 
    contribution patterns."</p>

    <p>The new platform leverages advanced analytics and proprietary algorithms to deliver:</p>
    <ul>
      <li>Real-time purr-formance monitoring across global teams</li>
      <li>Predictive insights for optimal box occupancy scheduling</li>
      <li>Strategic treat allocation recommendations</li>
      <li>Enhanced territorial coverage analysis</li>
    </ul>

    <p>Early adopters have reported remarkable results, with average improvements of 250% in 
    contribution consistency and 300% in developer satisfaction metrics.</p>

    <p>"The impact on our organization has been transformative," noted Sarah Chen, CTO of 
    Whiskers Corp. "Verdant™'s platform has revolutionized how we think about developer 
    productivity optimization."</p>

    <h3>Availability</h3>
    <p>The {} is available immediately to enterprise customers. 
    Pricing starts at $9,999 per month for the Professional tier, with custom Enterprise 
    packages available.</p>

    <h3>About Verdant™</h3>
    <p>Verdant™ is the world's leading enterprise feline productivity optimization platform, 
    trusted by Fortune 500 companies to maximize developer contribution quality and consistency. 
    Founded in 2025, Verdant™ serves over 10,000 organizations globally.</p>

    <h3>Media Contact</h3>
    <p>
      Mittens Patterson<br>
      VP of Strategic Communications<br>
      press@verdant.feline.corp<br>
      (555) 123-4567
    </p>
  </section>

  <footer>
    <p class="pr-footer">###</p>
  </footer>
</article>
"#,
            announcement.0,
            Utc::now().format("%B %d, %Y"),
            announcement.0,
            announcement.1,
            announcement.0
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/press-release-{}.html", timestamp);
        let commit_msg = format!(
            "Synergize {} press release to maximize stakeholder engagement and market penetration",
            announcement.0
        );

        (filename, content, commit_msg)
    }

    /// Generate meeting minutes
    fn generate_meeting_minutes(&mut self) -> (String, String, String) {
        let meetings = vec![
            "Feline Advisory Board - Weekly Sync",
            "Purr-formance Optimization Task Force",
            "Box Allocation Strategy Committee",
            "Enterprise Treat Distribution Council",
            "Territorial Coverage Planning Session",
        ];

        let meeting = meetings[self.rng.gen_range(0..meetings.len())];

        let content = format!(
            r#"<article class="meeting-minutes">
  <header>
    <h2>Meeting Minutes: {}</h2>
    <div class="meeting-meta">
      <div><strong>Date:</strong> {}</div>
      <div><strong>Time:</strong> 10:00 AM - 11:30 AM PST</div>
      <div><strong>Location:</strong> Executive Conference Room / Virtual</div>
    </div>
  </header>

  <section class="attendees">
    <h3>Attendees</h3>
    <ul>
      <li>Dr. Whiskers McFurrington (Chief Feline Officer)</li>
      <li>Mittens Patterson (VP of Purr-formance Engineering)</li>
      <li>Fluffy Anderson (Director of Box Optimization)</li>
      <li>Sparkles Chen (Senior Analytics Manager)</li>
      <li>Professor Tuna (Research & Development Lead)</li>
    </ul>
  </section>

  <section class="agenda">
    <h3>Agenda Items</h3>
    
    <div class="agenda-item">
      <h4>1. Q3 Purr-formance Review</h4>
      <p><strong>Discussion:</strong> Team reviewed Q3 contribution metrics, noting 23% improvement 
      in overall graph consistency. Strategic nap scheduling initiatives exceeded targets.</p>
      <p><strong>Action Items:</strong></p>
      <ul>
        <li>Mittens to prepare detailed analysis for board presentation (Due: Aug 15)</li>
        <li>Sparkles to update dashboards with latest metrics (Due: Aug 8)</li>
      </ul>
    </div>

    <div class="agenda-item">
      <h4>2. Box Occupancy Optimization Framework</h4>
      <p><strong>Discussion:</strong> Fluffy presented updated box allocation strategy. Proposal 
      to increase cardboard box budget by 40% to support territorial expansion.</p>
      <p><strong>Decision:</strong> Approved pending finance review.</p>
      <p><strong>Action Items:</strong></p>
      <ul>
        <li>Fluffy to submit budget proposal to CFO (Due: Aug 10)</li>
      </ul>
    </div>

    <div class="agenda-item">
      <h4>3. Treat Distribution Protocol Updates</h4>
      <p><strong>Discussion:</strong> Professor Tuna presented research findings on optimal treat 
      timing and frequency. Recommended implementation of AI-driven allocation system.</p>
      <p><strong>Action Items:</strong></p>
      <ul>
        <li>Professor Tuna to develop pilot program (Due: Aug 30)</li>
        <li>Mittens to coordinate with engineering teams (Due: Aug 20)</li>
      </ul>
    </div>

    <div class="agenda-item">
      <h4>4. Zoomie Detection Algorithm Enhancement</h4>
      <p><strong>Discussion:</strong> Engineering team reported 87% accuracy in burst activity 
      detection. Proposed machine learning improvements for next release.</p>
      <p><strong>Decision:</strong> Prioritize for Q4 roadmap.</p>
    </div>
  </section>

  <section class="next-steps">
    <h3>Next Meeting</h3>
    <p><strong>Date:</strong> {}<br>
    <strong>Time:</strong> 10:00 AM PST<br>
    <strong>Agenda:</strong> Budget review and Q4 planning</p>
  </section>

  <footer>
    <p class="minutes-footer">Minutes prepared by: Whiskers McFurrington | Approved: Pending</p>
  </footer>
</article>
"#,
            meeting,
            Utc::now().format("%B %d, %Y"),
            (Utc::now() + chrono::Duration::days(7)).format("%B %d, %Y")
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/minutes-{}.html", timestamp);
        let commit_msg = "Actualize strategic meeting minutes to facilitate cross-functional alignment and synergistic collaboration".to_string();

        (filename, content, commit_msg)
    }

    /// Generate metrics dashboard update
    fn generate_metrics_dashboard(&mut self) -> (String, String, String) {
        let score = self.rng.gen_range(75..98);
        let box_rate = self.rng.gen_range(60..95);
        let zoomie_idx = self.rng.gen_range(30..80);
        let treat_rate = self.rng.gen_range(85..100);

        let content = format!(
            r#"<article class="metrics-update">
  <header>
    <h2>Real-Time Metrics Dashboard Update</h2>
    <p class="subtitle">Live Purr-formance Intelligence</p>
    <p class="timestamp">Last Updated: {}</p>
  </header>

  <section class="metrics-snapshot">
    <div class="metrics-grid-large">
      <div class="metric-card-large">
        <h3>Purr-formance Score</h3>
        <div class="metric-value-large">{}</div>
        <div class="metric-trend positive">↑ 3.2% from last week</div>
      </div>
      
      <div class="metric-card-large">
        <h3>Box Occupancy Rate</h3>
        <div class="metric-value-large">{}%</div>
        <div class="metric-trend positive">↑ 5.7% from last week</div>
      </div>
      
      <div class="metric-card-large">
        <h3>Zoomie Frequency Index</h3>
        <div class="metric-value-large">{}</div>
        <div class="metric-trend neutral">→ Stable</div>
      </div>
      
      <div class="metric-card-large">
        <h3>Treat Conversion Rate</h3>
        <div class="metric-value-large">{}%</div>
        <div class="metric-trend positive">↑ 2.1% from last week</div>
      </div>
    </div>
  </section>

  <section class="insights-summary">
    <h3>Key Insights</h3>
    <ul>
      <li>Strategic nap scheduling continues to drive purr-formance improvements</li>
      <li>Box occupancy optimization yielding positive territorial coverage results</li>
      <li>Treat allocation efficiency at all-time high</li>
      <li>Zoomie patterns indicate healthy burst activity levels</li>
    </ul>
  </section>

  <section class="recommendations">
    <h3>Recommended Actions</h3>
    <ul>
      <li>Maintain current nap scheduling protocols</li>
      <li>Consider increasing box budget allocation by 15%</li>
      <li>Monitor zoomie patterns for potential optimization opportunities</li>
      <li>Continue strategic treat distribution framework</li>
    </ul>
  </section>

  <footer>
    <p class="auto-update">This dashboard updates automatically every 30 minutes via the Verdant™ Analytics Engine</p>
  </footer>
</article>
"#,
            Utc::now().format("%B %d, %Y at %H:%M UTC"),
            score,
            box_rate,
            zoomie_idx,
            treat_rate
        );

        let timestamp = Utc::now().timestamp();
        let filename = format!("frontend/insights/metrics-{}.html", timestamp);
        let commit_msg = "Optimize real-time purr-formance metrics dashboard to leverage data-driven strategic insights".to_string();

        (filename, content, commit_msg)
    }
}
