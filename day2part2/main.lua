-- parse a line of space-separated numbers into an array
function parse_report(line)
  local nums = {}
  for token in string.gmatch(line, "[^%s]+") do
    nums[#nums + 1] = tonumber(token)
  end
  return nums
end

function is_safe_report(report)
  local last_num = 0
  local current_diff = 0
  local last_diff = 0

  for i, num in ipairs(report) do
    if i == 1 then
      last_num = num
      goto continue
    end

    current_diff = num - last_num
    if current_diff == 0 then
      return false
    end
    if math.abs(current_diff) > 3 then
      return false
    end
    if current_diff > 0 and last_diff < 0 then
      return false
    end
    if current_diff < 0 and last_diff > 0 then
      return false
    end

    last_diff = current_diff
    last_num = num

    ::continue::
  end

  return true
end

function get_dampened_report(report, i)
  local j = 0
  local dampened_report = {}
  for k=1,#report do
    if k == i then goto continue end
    j = j + 1
    dampened_report[j] = report[k]
    ::continue::
  end
  return dampened_report
end

function get_dampened_reports(report)
  local dampened_reports = {}
  local j = 0
  for i=1,#report do
      dampened_reports[i] = get_dampened_report(report, i)
  end

  return dampened_reports
end

local safe_reports = 0

for line in io.lines("input.txt") do
  local report = parse_report(line)
  if is_safe_report(report) then
    safe_reports = safe_reports + 1
  else
    local dampened_reports = get_dampened_reports(report)
    for _, dampened_report in ipairs(dampened_reports) do
      if is_safe_report(dampened_report) then
        safe_reports = safe_reports + 1
        break
      end
    end
  end
end

print(safe_reports)
