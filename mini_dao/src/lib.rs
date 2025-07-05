use std::collections::HashMap;

// 提案
pub struct Proposal {
    pub describe: String,
    pub number: u32,
    pub status: Status,
}

impl Proposal {
    pub fn new(describe: String, number: u32, status: Status, map: &mut HashMap<u32, Proposal>) {
        map.insert(
            number,
            Proposal {
                describe,
                number,
                status,
            },
        );
    }

    pub fn close(number: u32, map: &mut HashMap<u32, Proposal>) -> Result<(), String> {
        let proposal = map.get_mut(&number).ok_or("未查寻到提案")?;
        proposal.status = Status::Close;
        Ok(())
    }
}

// 账号
pub struct Account {
    pub id: u32,
    pub name: String,
    pub role: Role,
}

impl Account {
    pub fn new(id: u32, name: String, role: Role) -> Self {
        Self { id, name, role }
    }

    pub fn vote(
        &mut self,
        number: u32,
        voting_result: &VotingResult,
        statistics: &mut Statistics,
        vota_map: &mut HashMap<u32, Proposal>,
    ) -> Result<(), String> {
        // 使用 get() 方法安全获取提案，不存在时返回错误
        let proposal = vota_map
            .get_mut(&number)
            .ok_or_else(|| format!("提案不存在: number={}", number))?;

        // 检查提案状态
        if proposal.status == Status::Close {
            return Err("提案已关闭，不能投票".to_string());
        }
        // 未初始化的时候get()找不到
        // let flag = statistics
        //     .vote_counts
        //     .get(&(number, self.id))
        //     .ok_or("查询失败")?;
        // if *flag {
        //     return Err("该账号和提案已经投过票".to_string());
        // }

        if statistics.vote_counts.contains_key(&(number, self.id)) {
            return Err("该账号和提案已经投过票".to_string());
        }

        // 投票记录
        statistics.vote_counts.insert((number, self.id), true);

        match statistics.counts.get_mut(&number) {
            Some(v) => match voting_result {
                VotingResult::Yes => {
                    let weight = self.role.weight();
                    v.agree += weight;
                    v.total += weight;
                }
                VotingResult::No => {
                    let weight = self.role.weight();
                    v.oppose += weight;
                    v.total += weight;
                }
            },
            None => {
                let weight = self.role.weight();
                let mut record = VoteRecord {
                    agree: 0,
                    oppose: 0,
                    total: 0,
                };
                match voting_result {
                    VotingResult::Yes => record.agree = weight,
                    VotingResult::No => record.oppose = weight,
                }
                record.total = weight;
                statistics.counts.insert(number, record);
            }
        }
        Ok(())
    }

    pub fn query_vote_statistics<'a>(
        &self,
        number: &u32,
        statistics: &'a Statistics,
    ) -> Result<&'a VoteRecord, String> {
        match statistics.counts.get(number) {
            Some(v) => return Ok(v),
            None => {}
        }
        Err(format!("未查到提案 {} 的投票统计", number))
    }
}

// 账户投票统计
pub struct Statistics {
    // 提案id 投票记录
    pub counts: HashMap<u32, VoteRecord>,
    // 是否已经投过票
    // 可以修改成:pub vote_counts: HashSet<(u32, u32)>
    // 通过statistics.vote_counts.contains_key(&(number, self.id))判断是否重复投票
    pub vote_counts: HashMap<(u32, u32), bool>,
}

// 提案和记录
pub struct VoteRecord {
    pub agree: u32,
    pub oppose: u32,
    pub total: u32,
}

// 状态
#[derive(PartialEq, Debug)]
pub enum Status {
    Open,
    Close,
}

// 权限
pub enum Role {
    Admin(u32),
    User(u32),
}

impl Role {
    fn weight(&self) -> u32 {
        match self {
            Role::Admin(w) => *w,
            Role::User(w) => *w,
        }
    }
}

// 投票结果
pub enum VotingResult {
    Yes,
    No,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_proposal_and_vote_success() {
        let mut proposals = HashMap::new();
        Proposal::new("提案1".to_string(), 1, Status::Open, &mut proposals);

        assert!(proposals.contains_key(&1));
        assert_eq!(proposals.get(&1).unwrap().status, Status::Open);

        let mut statistics = Statistics {
            counts: HashMap::new(),
            vote_counts: HashMap::new(),
        };

        let mut alice = Account::new(1001, "Alice".to_string(), Role::User(1));
        let result = alice.vote(1, &VotingResult::Yes, &mut statistics, &mut proposals);
        assert!(result.is_ok());

        let stat = statistics.counts.get(&1).unwrap();
        assert_eq!(stat.agree, 1);
        assert_eq!(stat.total, 1);
    }

    #[test]
    fn test_repeat_vote_should_fail() {
        let mut proposals = HashMap::new();
        Proposal::new("提案1".to_string(), 1, Status::Open, &mut proposals);

        let mut statistics = Statistics {
            counts: HashMap::new(),
            vote_counts: HashMap::new(),
        };

        let mut bob = Account::new(1002, "Bob".to_string(), Role::User(1));
        let r1 = bob.vote(1, &VotingResult::Yes, &mut statistics, &mut proposals);
        assert!(r1.is_ok());

        let r2 = bob.vote(1, &VotingResult::No, &mut statistics, &mut proposals);
        assert!(r2.is_err());
        assert_eq!(r2.unwrap_err(), "该账号和提案已经投过票");
    }

    #[test]
    fn test_vote_statistics_query() {
        let mut proposals = HashMap::new();
        Proposal::new("提案1".to_string(), 1, Status::Open, &mut proposals);

        let mut statistics = Statistics {
            counts: HashMap::new(),
            vote_counts: HashMap::new(),
        };

        let mut carol = Account::new(1003, "Carol".to_string(), Role::Admin(10));
        let _ = carol.vote(1, &VotingResult::Yes, &mut statistics, &mut proposals);

        let record = carol.query_vote_statistics(&1, &statistics);
        assert!(record.is_ok());
        let r = record.unwrap();
        assert_eq!(r.agree, 10);
        assert_eq!(r.total, 10);
    }

    #[test]
    fn test_vote_after_proposal_closed_should_fail() {
        let mut proposals = HashMap::new();
        Proposal::new("提案1".to_string(), 1, Status::Open, &mut proposals);
        Proposal::close(1, &mut proposals).unwrap();

        let mut statistics = Statistics {
            counts: HashMap::new(),
            vote_counts: HashMap::new(),
        };

        let mut dave = Account::new(1004, "Dave".to_string(), Role::User(1));
        let r = dave.vote(1, &VotingResult::Yes, &mut statistics, &mut proposals);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err(), "提案已关闭，不能投票");
    }
}
