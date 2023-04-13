function numberOfPairs(nums) {
    let result = [0, 0];

    let count = {};

    for (let i = 0; i < nums.length; i++) {
        if (count[nums[i]]) count[nums[i]]++
        else count[nums[i]] = 1
    }

    for (const [key, value] of Object.entries(count)) {
        result[0] += Math.floor(value / 2)
        result[1] += value % 2
    }

    return result
};

console.log(numberOfPairs([1,3,2,1,3,2,2]))