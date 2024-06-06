---
title: "The potential of mediation for free software development"
date: 2024-06-06
# weight: 1
tags: ["plotters"]
author: "Aaron Erhardt"
showToc: false
TocOpen: false
draft: false
hidemeta: false
comments: false
disableShare: false
disableHLJS: false
hideSummary: false
ShowReadingTime: true
ShowBreadCrumbs: true
ShowPostNavLinks: true
editPost:
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/tree/blog/content"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

Recently, I've been introduced to a communication technique called mediation, which is a process often used to avoid lawsuits or to resolve conflicts within companies or families.
Mediation is far better than other options in almost every single aspect and typically concludes with a win-win situation for all parties. 
This made me wonder whether some conflicts that happened in the free software community could have been avoided by utilizing this technique.
Therefore, in this blog post, I will have closer look at mediation and its potential specifically regarding free software development.
Even if you don't plan on using mediation yourself, there is a lot to be learned from its concepts.

> I wrote this article so that anyone should be able to follow it.
> I use some terminology that's only common in software development, but will try my best to explain everything upfront.

# What is free and open-source software?

When hearing about free software, most of us think about applications that are offered free of charge.
However, that differs from what most software developers would consider free software.
To explain the difference, many use the analogy of "free as in beer" vs. "free as in freedom".
A free beer is certainly nice, but you won't know how it was brewed and which ingredients were used.
It might contain something you would not expect nor want to drink, but you cannot know.
Similarly, software is something we use regularly, but we often do not know what's actually inside.
Applications are commonly shipped in formats that are easy to execute for a machine, but make it almost impossible for a human to reason about its functionality.
Still, we trust those applications with our personal data, private information and banking credentials.

On the side, free as freedom means that you have all the information about the software available.
You can read its code (recipe) and if you feel like it, build (brew) your own application (beer).
Moreover, the freedom contains the right to modify the code and publish those changes.
This not only makes the software trustworthy because malicious code would be hidden in plain sight, but also functions as a right to repair.
Whenever you notice a bug, you can take matters into your own hands and address it yourself.
Typically, such improvements are shared, which allows everyone else to benefit and makes the development efficient through collaboration.

> From now on, I will use the term "free" meaning "free as in freedom", which sometimes is also called "free and open-source".

While many are unaware of the type of software they are using, free software is inevitable.
There are a couple of popular free software projects, for example Linux, Firefox or LibreOffice.
Yet, almost every website or application uses some form of free software under the hood, even if they are not free themselves.
It is simply much more efficient to use existing code instead of writing everything yourself.
Of course, the value that free software creates cannot be measured directly because free software is almost always offered free of charge.
In a capitalistic society, this would suggest that it's worthless, but that clearly isn't the case.
Its value actually depends on the person using it and how they individually benefit from it.
Yet, because it's free and everyone can contribute instead of writing their own version, it saves countless hours of work.
Therefore, free software massively accelerates research, product and software development and growth of public infrastructure.
In my opinion, it generates more value than all big tech companies combined.

Something that's free and shared around the globe with groups coming together for vastly different reasons is obviously prone to conflicts.
However, free software thrives only when people come together to collaborate peacefully, whereas conflicts can drastically reduce the efficiency of the development.
Therefore, it's important to learn how to avoid conflicts or how to resolve them in a positive way, which is exactly where mediation comes in.

# Mediation

Mediation is a process in which an impartial third party, known as a mediator, helps disputing parties communicate and negotiate.
The goal is to reach the best possible solution for all parties.
Unlike a judge or arbitrator, the mediator does not make decisions or suggest solutions but uses dialogue, encourages understanding, and assists in identifying common ground.
Due to its confidential, collaborative, and cost-effective nature, this method is often used in various conflicts, including family, business, and community disputes.
By promoting open communication and focusing on the interests rather than the positions of the involved parties, mediation aims to produce agreements that satisfy all parties, thereby preserving relationships and preventing future conflicts.

In my own words, I would summarize the mindset behind mediation like this: "Let's not talk about what has happened, but find out want we really need and then figure out the best solution together".
Instead of focusing on the history of the conflict and who is responsible for what, it shifts the view to look forward to find ways out of the current situation.
It's also important to note that mediation is a voluntary process.
All parties agree to take part and can terminate the mediation at any point.
Even if the mediation fails, not much is lost and all other means of conflict resolution are still available. 

## Mediation process

Mediation is a structured process which follows several stages.
Although there are slightly different interpretations of the exact order of events, the overall structure of a mediation is always similar.

At the beginning of mediation, the mediator establishes a secure atmosphere and a common ground for the following conversations and the parties agree on the goals and define the structure of the mediation process.
In the next step, both parties explain the conflict from their perspective and collect the topics which are important to them.
The third step is about uncovering the underlying needs, desires and concerns behind the conflict.
By not just looking at the surface, but at the root of the conflict, effective ideas for resolving the problem can be developed.
The parties might have had several ideas how they would like to solve the issue beforehand, but only now, since they understand their own motivations and those of the other party, they are able to find the best possible solution.
After both parties have collected their ideas and have negotiated them, in the last step, the final terms are documented in an agreement.

# Types of conflicts resolution

To understand the value of mediation, it is important to know the alternatives.
Free software projects have unique ways of dealing with conflicts, especially if no agreement can be reached.
Depending on the severity of the disagreement, how large the project is and how many people are involved, different options are available.

## Creating a fork

In free software development, a fork occurs when a developer takes a copy of the source code of a software project and starts a new, independent project based on that code.
This new project, often also called "fork" or "downstream" project, can evolve in a different direction from the original project.
The ability to fork a project is in fact a fundamental aspect of free software development, upheld by open source licenses that guarantee the right to view, modify, and redistribute the code.
Most often, a fork is just used to publish changes which are intended to be merged into the original project, eventually.
However, when it comes to conflicts, a fork can also be used to start off an independent project.
Unless it is regularly kept in sync, the code of the fork will diverge from the upstream project over time.
Dividing the resources across two projects and additional efforts involved in managing and maintaining significantly reduce the efficiency compared to the previous joint development.
Therefore, it is rare that diverging forks coexists for a longer time and often all but one fork are discontinued.

## Re-implementation from scratch

Another option for conflict resolution is to start a new project from scratch instead of collaboration with an existing one.
If an existing code base is already old or uses technologies that are less desirable for a developer, this might appear as a better solution than forking.
Depending on the size of the previous project, this can be a major undertaking, though.
The downsides and motivations behind such a decision are, in general, very similar to a fork.

## Endless discussions

When two parties disagree on something strongly but without escalating the situations, it can happen that discussions continue without ever reaching a solution.
This binds valuable resources and thereby slows project on the development.
While no mediation is needed as long as this is still considered a respectful discussion, working out the underlying desires that lay at the root of the different viewpoints can help reach a constructive solution.

## Departure

Another type of conflict resolution is the departure of individual developers.
If a developer doesn't feel comfortable in a project anymore due to conflicts within the development team, they might decide to stop contributing and move to other projects.
This obviously reduces the development resources of the project and can also mean that important experts are no longer available.
Additionally, a toxic working environment can further reduce the efficiency of the developers.

# Causes of conflicts

Conflicts happen all the time around us, at work, within the family or with friends.
In each situation, different needs lay at the root and a different set of solutions is available.
Free software, too, is a special field when it comes to conflicts.
Therefore, I want to highlight some of the most common causes of conflicts in the free software community in the following section.

## Funding

One of the biggest challenges of free software development is without a doubt funding.
An astonishing amount of free software projects is run by a few volunteers, even if millions of developers and users rely on it daily.
This can lead to a disparity where large cooperations make money by using the free software without giving anything back to its developers.
Most of us would think that it is ethically unfair to profit from others without returning something, but profit-oriented cooperations have different incentives.
The conflict of interests gets even larger when the further development of a project relies on funding, for example, to pay the lead developers.

## Core believes

Many use and develop free software because they deeply value the freedom it creates and are consequently unwilling to make compromises in this regard.
By itself, this is a great trait and is something I think we need more of in our society.
It also establishes a common foundation for collaboration.
Yet, there are topics beyond this matter about which developers have very strong opinions.
Consequently, collaboration can be tough when the involved developers don't share the same ideas and would like to move a project in opposite directions.
Additionally, it can also happen that something technically irrelevant causes a conflict on a personal level that brings collaboration to a halt.

## Narcissism

Selfish or egocentric behavior is another common pattern in conflicts.
Developers who act narcissistically are often convinced that their approach is superior, disregarding the work and opinions of others.
It's hard to argue or to come to a solution with such people, therefore most often this attitude stops the collaboration already before it begins.

# Examples

In the following examples, I will have a look at past conflicts and try to assess whether mediation could have been helpful and wether there were consequences that could have been avoided.
Of course, I cannot accurately describe those conflicts in this article alone.
For some conflicts, there have been hundreds of Twitter and Reddit posts plus several articles that were written only on those topics alone.
Yet, I will give my best to summarize what happened.

## Libadwaita and System76

In 2022, the developers of the GNOME desktop, the most popular desktop environment on Linux, introduced a new user-interface library called libadwaita.
The goal of this library was to help with the creation of consistent and responsible graphical user interfaces.
After the introduction, System76, a hardware vendor that specializes in Linux devices, reacted surprised and displeased.
They had been using GNOME applications in their own Linux distribution called Pop!_OS, but introduced additional features like theming support.
With the introduction of libadwaita, they feared that theming and other customizations of their applications would no longer be possible.
While GNOME developers had purposefully removed the workaround that was commonly used to theme their applications, that did not mean that the applications could not be themed anymore and they even expressed an interest in working towards new methods for enabling theming.

Clearly, both parties had different goals of how customization of applications should look like and to which extend it should be possible.
They both wanted to bring the best possible experience to their users, but one side believed that extensive customization would be preferable, whereas the other side feared that too much customization could cause inconsistencies in applications in ways that the developers could hardly predict.
If they chose to utilize mediation to resolve the issue, I believe they could have come up with a solution that would have satisfied both sides.
As mentioned before, they both shared a common ground to provide the best experience for their users, which would have been a great starting point for conversations.
Additionally, the desires of both parties are not inherently incompatible, making it possible to find a solution through mediation.
For example, customizablilty could have been implemented in such a way, that one platform makes use of a strict set of hand-picked presets while others use the full set of possibilities, knowing that this could cause applications to behave unpredictably.

Unfortunately, instead of seeking a dialog, both parties called each other out on social media, pointing to the mistakes of the opposite party.
Eventually, System76 chose to abandon the GNOME desktop and its ecosystem entirely.
To this day, the new desktop environment of System76 is still in development, whereas the GNOME developers have not moved forward with any plans to improve customizability.

## The Rust foundation trademarks 

In 2023, the Rust Rust Foundation, which backs the development of the popular Rust programming language, introduced a draft for a new trademark policy, which added restrictions for using the name and the logo of the language.
Many in the community perceived the new restrictions as overly restrictive and feared more invasive steps of the foundation.
In response, a group of developers decided to fork the Rust language, creating Crablang to preserve a more open and collaborative environment without the proposed constraints.
However, the foundation responded soon with a statement clarifying the intention of the draft as a means of collecting feedback.
They actively reached out to the community for feedback and provided transparency to their process, eventually addressing all points of criticism.
Ultimately, all concerns were addressed and the fork was discontinued shortly after.

While in this case no mediation was used, the actions taken by the Rust foundation are similar and show that the steps used in a mediation can be effective on their own.
Most importantly, the foundation kept respectful communication with members of the community, identified their needs and together they found a solution that satisfied both sides.

## Audacity's telemetry controversy

In 2021, Audacity, a popular open-source audio editing software, was acquired by a company called Muse Group.
Shortly after, the new governance planned to introduce telemetry features based on Google Analytics and to raise the minimum age to 13 years.
The free software community immediately raised privacy concerns, fearing unwarranted data collection and misuse.
By the time that Muse Group released a response that they would significantly reduce the amount of data collected and would drop the age restriction, there were already several active forks of Audacity.
Due to the slow response, the trust couldn't be regained anymore and one of the forks called Tenacity remains active to this day.

I believe, if Muse Group had been open for a mediation with those criticizing their plans, they would have been able to respond much quicker and keep the trust intact.
Clearly, at the core of the issue was a lack of understanding of the needs of the free software community.
They did not anticipate such a backlash, but if they figured out the motives behind it earlier and worked together with the critics on a solution, I think all previous developers of Audacity would still work together on one project.

# Conclusion

In summary, mediation has great potential to resolve conflicts in the free software community.
It is a great tool for coming up with a constructive solution and keeping good relationships after the original issue.
Moreover, if both parties do still communicate with each other respectfully, many aspects of mediation can be applied to resolve a disagreement even without going through a full mediation.
Just knowing about this technique has changed my way of looking at conflicts and it makes me even sadder when I see conflicts escalate unnecessarily.
I think all of use in the free software community share the goal to provide great free software.
We might disagree sometimes, but I believe we can overcome our conflicts and create awesome software together.

## Sources
 
> ### Mediation
>
> + C. S. Rabe und M. Wode, Mediation. Springer, 2020.
> + M. Koschany-Rohbeck, Praxishandbuch Wirtschaftsmediation. Springer Gabler, 2018.
> + H. Kamp, Friedensstifter und Vermittler im Mittelalter. WBG Academic, 2001.
>
> ### Other
>
> + [GNOME and System76](https://blogs.gnome.org/christopherdavis/2021/11/10/system76-how-not-to-collaborate/)
> + [Rust foundation trademark](https://foundation.rust-lang.org/news/rust-trademark-policy-draft-revision-next-steps/)
> + [Audacity fork](https://codeberg.org/tenacityteam/tenacity#motivation)