use std::path::PathBuf;

use anyhow::Result;

pub struct Agent {
    pub label: String,
    pub command: AgentCommand,
    pub launch_type: LaunchType,
}

impl Agent {
    pub fn from_args(args: &clap::ArgMatches) -> Result<Agent> {
        todo!()
    }
}

pub enum AgentCommand {
    SimpleCommand { path: PathBuf },
    CommandWithArgs { path: PathBuf, args: Vec<String> },
}

pub enum LaunchType {
    OnLoad,
    Interval { seconds: u32 },
    ScheduledIntervals { intervals: Vec<Interval> },
}

pub struct Interval {
    pub month: Option<u32>,
    pub day: Option<u32>,
    pub weekday: Option<u32>,
    pub hour: Option<u32>,
    pub minute: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct PList(pub String);

impl PList {
    pub fn from_agent(agentspec: &Agent) -> Result<PList> {
        todo!()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_plist()  {
        let input = Agent {
            label: "local.test-agent".into(),
            command: AgentCommand::SimpleCommand {
                path: PathBuf::from("/usr/bin/true"),
            },
            launch_type: LaunchType::OnLoad,
        };

        let res: String = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
	<dict>
		<key>Label</key>
		<string>local.test-agent</string>
		<key>Program</key>
		<string>/usr/bin/true</string>
		<key>RunAtLoad</key>
		<true/>
	</dict>
</plist>"#.into();

        assert_eq!(
            PList::from_agent(&input)
                .expect("Generation for valid agent spec should succeed")
                .0,
            res
        )
    }
}
