While debugging some flaky CI behavior I wanted to see what happens in GitHub Actions runners when you run out of disk space.  So I wrote this very simple program to run it out of space and see what happens.

On Mac, [this failed pretty crisply](https://github.com/davepacheco/github-ci-disk-exhaustion/runs/7457954518?check_suite_focus=true#step:4:214): an explicit ENOSPC exactly when you'd expect it to.  There's no increase in write latency leading up to it.  The job failed immediately after the ENOSPC and the full log is reported in the web UI.

On Linux, [this failed very badly](https://github.com/davepacheco/github-ci-disk-exhaustion/runs/7457954423?check_suite_focus=true).  Everything was fine for a while until it very suddenly wasn't.  I had log entries up to that point showing no increase in latency.  But after that point, there was no output for 30+ minutes, then the job failed, and now I have _no_ logs (not even the ones I could see while the job was running).  On the [summary page](https://github.com/davepacheco/github-ci-disk-exhaustion/actions/runs/2714703018) (which I don't usually think to check) I see "Hosted Agent lost communication with the server. Anything in your workflow that terminates the runner process, starves it for CPU/Memory, or blocks its network access can cause this error."

I'm not sure how generalizable this result is -- it might depend on things like how big the writes are and just how close to the limit you get -- but this seemed like useful data that I figured I'd share.  I guess the point is: "don't run out of disk space in GitHub Actions" but it's hard to avoid that too.
